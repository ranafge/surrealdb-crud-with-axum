

pub async fn handle(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("create_entry")))
}


// setting tracing  

tracing_subscriber::fmt::init();

let mut service = ServiceBuilder::new()
    .layer(TraceLayer::new_for_http())
    .service_fn(handle);

let request = Request::new(Body::from("foo"));

let response = service
    .ready()
    .await?
    .call(request)
    .await?;