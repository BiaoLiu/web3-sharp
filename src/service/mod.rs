use std::sync::Arc;

use crate::conf;
use crate::data;
use crate::service::auth::AuthService;
use crate::service::product::ProductService;
use crate::service::user::UserService;

pub mod auth;
pub mod ethereum;

pub mod product;
pub mod user;

#[derive(Clone)]
pub struct Service {
    pub auth_service: AuthService,
    pub product_service: ProductService,
    pub user_service: UserService,
}

impl Service {
    pub fn new(conf: Arc<conf::Config>, data: Arc<data::Data>) -> Self {
        let auth_service = AuthService::new(conf.clone(), data.clone());
        let product_service = ProductService::new(conf.clone(), data.clone());
        let user_service = UserService::new(conf, data);
        Service {
            auth_service,
            product_service,
            user_service,
        }
    }
}
