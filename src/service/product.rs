use crate::{conf, data};
use sea_orm::{ColumnTrait, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter, QueryOrder};
use std::sync::Arc;

use crate::data::entity::{
    prelude::*,
    user_product::{self, UserProductModel},
};
use crate::error::Error;

#[derive(Clone)]
pub struct ProductService {
    conf: Arc<conf::Config>,
    data: Arc<data::Data>,
}

impl ProductService {
    pub fn new(conf: Arc<conf::Config>, data: Arc<data::Data>) -> Self {
        ProductService { conf, data }
    }

    pub async fn get_user_product(
        &self,
        user_id: i64,
        user_product_id: i64,
    ) -> Result<UserProductModel, Error> {
        let (mut user, product) = UserProduct::find()
            .find_also_related(Product)
            .filter(user_product::Column::UserId.eq(user_id))
            .filter(user_product::Column::Id.eq(user_product_id))
            .one(&self.data.db)
            .await?
            .ok_or_else(|| Error::NotFound("user product not found".to_string()))?;
        user.product = product;
        Ok(user)
    }

    pub async fn get_user_products(
        &self,
        user_id: i64,
        page_num: u64,
        page_size: u64,
    ) -> Result<(Vec<UserProductModel>, u64), Error> {
        let mut res = Vec::new();
        let paginator = UserProduct::find()
            .filter(user_product::Column::UserId.eq(user_id))
            .order_by_desc(user_product::Column::ProductId)
            .paginate(&self.data.db, page_size);
        let page_total = paginator.num_pages().await?;
        let user_products = paginator.fetch_page(page_num - 1).await?;

        let products = user_products.load_one(Product, &self.data.db).await?;
        for (mut user_product, product) in user_products.into_iter().zip(products.into_iter()) {
            user_product.product = product;
            res.push(user_product);
        }
        Ok((res, page_total))
    }
}
