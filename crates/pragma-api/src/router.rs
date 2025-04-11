use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

use utoipa::OpenApi as OpenApiT;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

pub fn api_router<T: OpenApiT>(_state: AppState) -> Router<AppState> {
    let open_api = T::openapi();
    Router::new()
        .route("/health", get(health))
        .merge(SwaggerUi::new("/v1/docs").url("/v1/docs/openapi.json", open_api))
        .fallback(handler_404)
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
