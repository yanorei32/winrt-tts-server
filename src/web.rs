use crate::model::{ApiRequest, Voice};
use axum::{
    extract::Json,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {}

async fn root_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html")],
        include_str!("../assets/index.html"),
    )
}

async fn voices_handler() -> Json<&'static [Voice]> {
    Json(crate::speech::voices())
}

async fn tts_handler(Json(req): Json<ApiRequest>) -> impl IntoResponse {
    match crate::speech::synthesis(&req).await {
        Err(e) => {
            tracing::warn!("{}", e);

            let status_code = match e.is_client_error() {
                true => StatusCode::BAD_REQUEST,
                false => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status_code, e.to_string()).into_response()
        }
        Ok(buffer) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "audio/wav".parse().unwrap());
            (headers, buffer).into_response()
        }
    }
}

pub async fn serve(listener: TcpListener) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api/tts", post(tts_handler))
        .route("/api/voices", get(voices_handler))
        .with_state(AppState {});

    axum::serve(listener, app).await
}
