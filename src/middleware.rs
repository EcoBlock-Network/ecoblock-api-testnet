use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::response::Response;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub fn create_middleware_stack() -> tower::ServiceBuilder<
    tower::layer::util::Stack<tower_http::cors::CorsLayer, tower::layer::util::Identity>,
> {
    ServiceBuilder::new().layer(
        CorsLayer::new()
            .allow_origin(
                ["http://localhost:5173", "http://localhost:3000"]
                    .iter()
                    .map(|s| s.parse::<HeaderValue>().unwrap())
                    .collect::<Vec<_>>(),
            )
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
                axum::http::header::ACCEPT,
            ])
            .allow_credentials(true),
    )
}

pub async fn logging_middleware(
    req: Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = std::time::Instant::now();

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    Ok(response)
}
