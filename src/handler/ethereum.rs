use crate::service::ethereum::EthereumService;
use axum::body::Body;
use axum::{http::Request, Extension};

pub async fn notify(Extension(ethereum_service): Extension<EthereumService>, req: Request<Body>) {
    // let (parts, body) = req.into_parts();

    // let bytes = match hyper::body::to_bytes(body).await {
    //     Ok(bytes) => bytes,
    //     Err(err) => {
    //         return Err((
    //             StatusCode::BAD_REQUEST,
    //             format!("failed to read {direction} body: {err}"),
    //         ));
    //     }
    // };
    //
    // if let Ok(body) = std::str::from_utf8(&bytes) {
    //     tracing::debug!("{direction} body = {body:?}");
    // }
}
