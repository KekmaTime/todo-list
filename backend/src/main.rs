#[macro_use]
extern crate rocket;

use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::{State, response::status::Custom, http::Status};
use tokio_postgres::{Client, NoTls};
use rocket_cors::{CorsOptions, AllowedOrigins};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: Option<i32>,
    title: String,
    description: String,
    completed: bool,
}

#[post("/api/todos", data = "<todo>")]
async fn add_todo(
    conn: &State<Client>,
    todo: Json<Todo>,
) -> Result<Json<Todo>, Custom<String>> {
    execute_query(
        conn,
        "INSERT INTO todos (title, description, completed) VALUES ($1, $2, $3) RETURNING id, title, description, completed",
        &[&todo.title, &todo.description, &todo.completed],
    )
    .await?
    .get(0)
    .ok_or_else(|| Custom(Status::InternalServerError, "Failed to insert todo".to_string()))
    .map(|row| Json(Todo {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        completed: row.get(3),
    }))
}

#[get("/api/todos")]
async fn get_todos(conn: &State<Client>) -> Result<Json<Vec<Todo>>, Custom<String>> {
    get_todos_from_db(conn).await.map(Json)
}

async fn get_todos_from_db(client: &Client) -> Result<Vec<Todo>, Custom<String>> {
    let todos = client
        .query("SELECT id, title, description, completed FROM todos", &[])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
        .into_iter()
        .map(|row| Todo {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
            completed: row.get(3),
        })
        .collect::<Vec<Todo>>();

    Ok(todos)
}

#[put("/api/todos/<id>", data = "<todo>")]
async fn update_todo(
    id: i32,
    todo: Json<Todo>,
    conn: &State<Client>,
) -> Result<Json<Todo>, Custom<String>> {
    execute_query(
        conn,
        "UPDATE todos SET title = $1, description = $2, completed = $3 WHERE id = $4 RETURNING id, title, description, completed",
        &[&todo.title, &todo.description, &todo.completed, &id],
    )
    .await?
    .get(0)
    .ok_or_else(|| Custom(Status::NotFound, "Todo not found".to_string()))
    .map(|row| Json(Todo {
        id: row.get(0),
        title: row.get(1),
        description: row.get(2),
        completed: row.get(3),
    }))
}

#[delete("/api/todos/<id>")]
async fn delete_todo(
    conn: &State<Client>,
    id: i32,
) -> Result<Status, Custom<String>> {
    execute_query(
        conn,
        "DELETE FROM todos WHERE id = $1",
        &[&id],
    )
    .await?;
    Ok(Status::NoContent)
}

async fn execute_query(
    client: &Client,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<tokio_postgres::Row>, Custom<String>> {
    client.query(query, params)
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

#[launch]
async fn rocket() -> _ {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls)
        .await
        .expect("Failed to connect to database");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Database connection error: {}", e);
        }
    });

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id SERIAL PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT false
            )",
            &[],
        )
        .await
        .expect("Failed to create table");

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error while building CORS");

    rocket::build()
        .manage(client)
        .mount("/", routes![add_todo, get_todos, update_todo, delete_todo])
        .attach(cors)
}