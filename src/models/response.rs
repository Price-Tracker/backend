use crate::models::category::Category;
use crate::models::product::{ProductDTO, ProductStoreDTO};
use crate::models::user::{
    HistoryWithProductDTO, PasswordRequirements, UserShoppingCartDTO, UserSubscribedProductDTO,
};
use crate::models::user_tokens::UserTokensDTO;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases(
    ResponseTokens = ResponseBody<UserTokensDTO>,
    ResponsePasswordRequirements = ResponseBody<PasswordRequirements>,
    ResponseLogin = ResponseBody<String>,
    ResponseProduct = ResponseBody<ProductDTO>,
    ResponseProductStore = ResponseBody<ProductStoreDTO>,
    ResponseProductSubscription = ResponseBody<UserSubscribedProductDTO>,
    ResponseVecProduct = ResponseBody<Vec<ProductDTO>>,
    ResponseVecShoppingCart = ResponseBody<Vec<UserShoppingCartDTO>>,
    ResponseVecCategory = ResponseBody<Vec<Category>>,
    ResponseVecHistory = ResponseBody<Vec<HistoryWithProductDTO>>,
    ResponseSubscriptions = ResponseBody<Vec<UserSubscribedProductDTO>>,
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
