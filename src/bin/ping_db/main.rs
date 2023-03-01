use lambda_http::aws_lambda_events::serde_json;
use lambda_http::{service_fn, Body, Error, Request, Response};
use sqlx::{Connection, Executor, MySqlConnection};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;

type DbConnectionHandle = Arc<RwLock<MySqlConnection>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL IS NOT PROVIDED");
    let db_connection_handle: DbConnectionHandle =
        Arc::new(RwLock::new(MySqlConnection::connect(&db_url).await?));
    lambda_http::run(service_fn(|request: Request| {
        ping_db(request, db_connection_handle.clone())
    }))
    .await?;

    Ok(())
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct MyBody {
    pub rows_affected: u64,
    pub response: String,
}

async fn ping_db(
    _request: Request,
    db_connection_handle: DbConnectionHandle,
) -> Result<Response<Body>, Error> {
    let mut db = db_connection_handle.write().await;
    let result = db
        .execute(sqlx::query("SELECT * FROM recruiter LIMIT 1"))
        .await?;
    let rows_affected = result.rows_affected();
    let body = MyBody {
        rows_affected,
        response: String::from("Pong"),
    };
    let res = serde_json::to_string(&body)?;
    Ok(Response::builder().status(200).body(res.into())?)
}
