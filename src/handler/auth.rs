use std::time::SystemTime;

use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::app::response::JsonResponse;
use crate::error::Error;
use crate::service::{auth, Service};

#[derive(Deserialize, Debug)]
pub struct LoginReq {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResp {
    username: String,
    token: String,
}

#[derive(Deserialize, Debug)]
pub struct AccountReq {
    id: i64,
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct GetUserListReq {
    username: String,
    role: Option<String>,
}

pub async fn login(
    Extension(service): Extension<Service>,
    Json(req): Json<LoginReq>,
) -> Result<JsonResponse<LoginResp>, Error> {
    println!("req:{:?}", req);

    // now time
    let now = SystemTime::now();

    let login_req = auth::LoginReq {
        username: req.username,
        password: req.password,
    };
    let m = service.auth_service.login(&login_req).await?;

    let login_res = LoginResp {
        username: m.username,
        token: "123456".to_string(),
    };
    Ok(JsonResponse::success(login_res))
}

pub async fn register(
    Extension(service): Extension<Service>,
    Json(req): Json<AccountReq>,
) -> Result<JsonResponse<LoginResp>, Error> {
    println!("req:{:?}", req);

    let register_req = auth::UserReq {
        username: req.username,
        password: req.password,
    };
    let m = service.auth_service.register(&register_req).await?;

    let res = LoginResp {
        username: m.username,
        token: "123456".to_string(),
    };
    Ok(JsonResponse::success(res))
}

pub async fn update_user(
    Extension(service): Extension<Service>,
    Json(req): Json<AccountReq>,
) -> Result<JsonResponse<()>, Error> {
    println!("req:{:?}", req);

    let account_req = auth::UserReq {
        username: req.username,
        password: req.password,
    };
    service.auth_service.update_user(req.id, &account_req).await?;
    Ok(JsonResponse::success(()))
}

pub async fn get_user_list(
    Extension(service): Extension<Service>,
    Json(req): Json<GetUserListReq>,
) -> Result<JsonResponse<Vec<LoginResp>>, Error> {
    let user_req = auth::GetUserListReq {
        username: req.username,
        role: req.role,
    };
    let users = service.auth_service.get_user_list(&user_req).await?;

    let mut res = Vec::new();
    for user in users {
        let login_res = LoginResp {
            username: user.username,
            token: "123456".to_string(),
        };
        res.push(login_res);
    }
    Ok(JsonResponse::success(res))
}
