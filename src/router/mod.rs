use crate::handler;
use crate::service::Service;
use axum::routing::post;
use axum::{Extension, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub fn init_router(service: Service) -> Router {
    Router::new()
        .route("/login", post(handler::auth::login))
        .route("/register", post(handler::auth::register))
        .route("/update-user", post(handler::auth::update_user))
        .route("/get-user-list", post(handler::auth::get_user_list))
        .route(
            "/product/get-user-product",
            post(handler::product::get_user_product),
        )
        .route(
            "/product/get-user-products",
            post(handler::product::get_user_products),
        )
        .layer(
            //     TraceLayer::new_for_http()
            //         .make_span_with(|request: &Request<_>| {
            //             let trace_id = request.extensions().get::<TraceId<String>>().unwrap();
            //
            //             info_span!("http_request", trace_id = trace_id)
            //         })
            // )
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(Extension(service))
}
