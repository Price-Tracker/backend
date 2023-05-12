use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserTokensDTO {
    pub access_token: String,
    pub refresh_token: String
}