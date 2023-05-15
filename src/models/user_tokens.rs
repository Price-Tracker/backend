use crate::config::app::Config;
use crate::middlewares::jwt_middleware::TokenClaims;
use crate::models::user::User;
use crate::schema::user_tokens::{self, dsl::*};
use actix_web::web::Data;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::EncodingKey;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Identifiable, Queryable)]
pub struct UserToken {
    pub id: i32,
    pub user_id: i32,
    pub refresh_token: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = user_tokens)]
pub struct UserTokenInsertable {
    pub user_id: i32,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserTokensDTO {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserRefreshTokenDTO {
    pub refresh_token: String,
}

impl UserToken {
    pub fn generate_access_token(user: User, config: Data<Config>) -> String {
        let now = Utc::now();

        let token_claims = TokenClaims {
            sub: user.id.to_string(),
            iat: now.timestamp() as usize,
            exp: (now + Duration::seconds(config.jwt_expires_in_secs as i64)).timestamp() as usize,
            nickname: user.nickname,
            login: user.login,
            roles: vec![],
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &token_claims,
            &EncodingKey::from_secret(config.jwt_secret.as_ref()),
        )
        .unwrap();

        token
    }

    pub fn generate_refresh_token(conn: &mut PgConnection, user: User) -> String {
        let token: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        let user_token = UserTokenInsertable {
            user_id: user.id,
            refresh_token: token.clone(),
        };
        diesel::insert_into(user_tokens)
            .values(user_token)
            .execute(conn)
            .unwrap();

        token
    }

    pub fn find_refresh_token(conn: &mut PgConnection, token: String) -> QueryResult<UserToken> {
        user_tokens
            .filter(refresh_token.eq(token))
            .get_result::<UserToken>(conn)
    }

    pub fn refresh_tokens(
        conn: &mut PgConnection,
        user_token: UserToken,
        config: Data<Config>,
    ) -> UserTokensDTO {
        let user = User::find_user_by_id(conn, user_token.user_id)
            .expect("Undefined behavior on the DB side");

        diesel::delete(user_tokens.filter(refresh_token.eq(user_token.refresh_token)))
            .execute(conn)
            .unwrap();

        let new_access_token = UserToken::generate_access_token(user.clone(), config);
        let new_refresh_token = UserToken::generate_refresh_token(conn, user.clone());

        let new_user_tokens = UserTokensDTO {
            access_token: new_access_token,
            refresh_token: new_refresh_token,
        };

        new_user_tokens
    }
}
