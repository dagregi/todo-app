use axum::{
    extract::{Path, State},
    Form, Json,
};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub completed: bool,
}

// fetch all todos from db
pub async fn read(State(pool): State<PgPool>) -> Result<Json<Vec<Todo>>> {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;

    Ok(Json(todos))
}

// struct for the new todo
#[derive(Serialize, Deserialize)]
pub struct NewTodo {
    pub description: String,
}
// method to create new todos
pub async fn create(State(pool): State<PgPool>, Form(new_todo): Form<NewTodo>) -> Result<String> {
    sqlx::query!(
        "INSERT INTO todos (description) VALUES ($1)",
        &new_todo.description,
    )
    .execute(&pool)
    .await?;

    Ok("Success!".to_string())
}

// delete todos by id
pub async fn delete_todo(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<String> {
    sqlx::query!("DELETE FROM todos WHERE id=$1", &id)
        .execute(&pool)
        .await?;

    Ok("Success!".to_string())
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodo {
    pub description: String,
    pub completed: bool,
}
// method to update todos
pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Form(new_todo): Form<UpdateTodo>,
) -> Result<String> {
    sqlx::query!(
        "UPDATE todos SET description=$2, completed=$3  WHERE id=$1",
        &id,
        &new_todo.description,
        &new_todo.completed
    )
    .execute(&pool)
    .await?;

    Ok("Success".to_string())
}
