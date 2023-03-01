use lambda_http::{service_fn, Body, Error, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(|request: Request| {
        ping(request)
    })).await?;

    Ok(())
}

async fn ping(_request: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(200).body("Pong".into())?)
}
