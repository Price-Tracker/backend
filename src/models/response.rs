use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::models::user_tokens::UserTokensDTO;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases(
    ResponseTokens = ResponseBody<UserTokensDTO>,
    ResponseLogin = ResponseBody<String>
)]
pub struct ResponseBody<T> {
    pub status: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(status: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            status: status.to_string(),
            data,
        }
    }
}
