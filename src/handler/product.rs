use axum::{Extension, Json};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::app::response::JsonResponse;
use crate::data::entity::{product::ProductModel, user_product::UserProductModel};
use crate::error::Error;
use crate::service::Service;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetUserProductReq {
    pub user_id: i64,
    pub user_product_id: i64,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductResp {
    pub id: i64,
    pub product_name: String,
    pub picture: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetUserProductsReq {
    pub user_id: i64,
    pub page_num: u64,
    pub page_size: u64,
}

impl From<ProductModel> for ProductResp {
    fn from(value: ProductModel) -> Self {
        Self {
            id: value.id,
            product_name: value.product_name,
            picture: value.picture,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserProductResp {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub status: i8,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub product: Option<ProductResp>,
}

impl From<UserProductModel> for UserProductResp {
    fn from(value: UserProductModel) -> Self {
        let product = value.product.map(|p| p.into());

        Self {
            id: value.id,
            user_id: value.user_id,
            product_id: value.product_id,
            status: value.status,
            created_at: value.created_at,
            updated_at: value.updated_at,
            product,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserProductListResp {
    pub list: Vec<UserProductResp>,
    pub page_total: u64,
}

pub async fn get_user_product(
    Extension(service): Extension<Service>,
    Json(req): Json<GetUserProductReq>,
) -> Result<JsonResponse<UserProductResp>, Error> {
    let res = service
        .product_service
        .get_user_product(req.user_id, req.user_product_id)
        .await?;
    Ok(JsonResponse::success(res.into()))
}

pub async fn get_user_products(
    Extension(service): Extension<Service>,
    Json(req): Json<GetUserProductsReq>,
) -> Result<JsonResponse<UserProductListResp>, Error> {
    let user_products = service
        .product_service
        .get_user_products(req.user_id, req.page_num, req.page_size)
        .await?;

    let mut user_product_resps = Vec::new();
    for user_product in user_products.0 {
        user_product_resps.push(user_product.into());
    }

    let res = UserProductListResp {
        list: user_product_resps,
        page_total: user_products.1,
    };
    Ok(JsonResponse::success(res))
}
