use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

use crate::conf;
use crate::data::entity::{
    prelude::*,
    user::{self, UserModel},
};
use crate::{data, error::Error};

pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UserReq {
    pub username: String,
    pub password: String,
}

pub struct GetUserListReq {
    pub username: String,
    pub role: Option<String>,
}

#[derive(Clone)]
pub struct AuthService {
    conf: Arc<conf::Config>,
    data: Arc<data::Data>,
}

impl AuthService {
    pub fn new(conf: Arc<conf::Config>, data: Arc<data::Data>) -> Self {
        AuthService { conf, data }
    }
    pub async fn login(&self, req: &LoginReq) -> Result<UserModel, Error> {
        User::find()
            .filter(user::Column::Username.eq(&req.username))
            .one(&self.data.db)
            .await
            .context("login error")?
            .ok_or_else(|| Error::NotFound("user not found".to_string()))
    }

    pub async fn register(&self, req: &UserReq) -> Result<UserModel, Error> {
        let count = User::find()
            .filter(user::Column::Username.eq(&req.username))
            .count(&self.data.db)
            .await?;
        if count > 0 {
            return Err(Error::InternalServerError(
                "user already exists".to_string(),
            ));
        }

        let a = user::ActiveModel {
            username: Set(req.username.to_owned()),
            password: Set(req.password.to_owned()),
            ..Default::default()
        };
        let res = a.insert(&self.data.db).await?;
        println!("last id {}", res.user_id);
        Ok(res)
    }

    pub async fn update_user(&self, id: i64, req: &UserReq) -> Result<(), Error> {
        let a = User::find_by_id(id)
            .one(&self.data.db)
            .await?
            .ok_or_else(|| Error::NotFound("user not found".to_string()))?;

        let mut am: user::ActiveModel = a.into();
        am.username = Set(req.username.to_owned());
        am.update(&self.data.db).await?;
        Ok(())
    }

    pub async fn get_user_list(&self, req: &GetUserListReq) -> Result<Vec<UserModel>, Error> {
        let s = User::find().filter(user::Column::Username.like(&req.username));

        // if let Some(role) = &req.role {
        //     s = s.filter(user::Column::Role.eq(role));
        // }
        let res = s.all(&self.data.db).await?;
        Ok(res)
    }
}
