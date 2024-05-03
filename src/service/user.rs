use std::sync::Arc;

use crate::{conf, data};

#[derive(Clone)]
pub struct UserService {
    conf: Arc<conf::Config>,
    data: Arc<data::Data>,
}

impl UserService {
    pub fn new(conf: Arc<conf::Config>, data: Arc<data::Data>) -> Self {
        UserService { conf, data }
    }
}
