use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct JsonResponse<T> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> JsonResponse<T>
where
    T: Serialize,
{
    #[allow(unused)]
    pub fn new(code: u16, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    #[allow(unused)]
    pub fn success(data: T) -> Self {
        Self::new(StatusCode::OK.as_u16(), "success".to_string(), Some(data))
    }

    #[allow(unused)]
    pub fn fail(code: u16, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}

impl<T> IntoResponse for JsonResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let res = serde_json::to_string(&self);
        let s = match res {
            Ok(str) => str,
            Err(_) => "".to_string(),
        };
        (StatusCode::OK, s).into_response()
    }
}
