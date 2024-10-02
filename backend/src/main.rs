#[macro_use]
extern crate rocket;

use rocket::serde::{Serialize,Deserialize,json::Json};
use rocket::{State, response::status::Custom, http::Status};
use tokio_postgres::{Client, NoTls};
use rocket_cors::{ CorsOptions, AllowedOrigins};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

#[post("api/users", data = "<user>")]
async fn add_user(
    conn: &State<Client>,
    user: Json<User>,
) -> Result<Json<User>, Custom<String>> {
    execute!(
        conn,
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email]
    )
    .await?;
    get_users(conn).await
}

async fn execute_query(
    client: &Client,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<64, Custom<String>> {
    client.execute(query, params)
    .await
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    Ok(())  
}

