use lambda_http::aws_lambda_events::serde_json;
use lambda_http::{service_fn, Body, Error, Request, Response};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL IS NOT PROVIDED");
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .connect(db_url.as_str())
        .await?;
    lambda_http::run(service_fn(|request: Request| ping_db(request, &pool))).await?;

    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct MyBody {
    pub recruiters: Vec<Recruiter>,
    pub response: String,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
struct Recruiter {
    pub name: String,
    pub last_name: String,
    pub email: String,
}

async fn ping_db(_request: Request, pool: &Pool<MySql>) -> Result<Response<Body>, Error> {
    let recruiters = sqlx::query_as::<_, Recruiter>(
        "SELECT r.name, r.last_name, r.email FROM recruiter AS r LIMIT 10",
    )
    .fetch_all(pool)
    .await?;

    let body = MyBody {
        recruiters,
        response: String::from("Pong"),
    };
    let res = serde_json::to_string(&body)?;
    Ok(Response::builder().status(200).body(res.into())?)
}
