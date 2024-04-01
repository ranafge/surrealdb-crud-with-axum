use axum:: {
    extract::Request, http::HeaderValue, middleware:: Next, response::Response
};

pub async fn my_middleware( request: Request, next: Next) -> Response{
    let mut response = next.run(request).await;
    tracing::info!("Access-Control-Allow-Origin inserting");
    response.headers_mut().insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_static("*"),
    );
    response.headers_mut().insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, OPTIONS, PUT, PATCH, DELETE"),
    );
    response.headers_mut().insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("X-Requested-With,content-type"),
    );
    response
}