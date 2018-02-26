use rocket_contrib::{Json, Value};

use super::super::database;
use super::super::models::{NewTodo, Todo};

#[post("/", format = "application/json", data = "<todo_data>")]
fn new(todo_data: Json<NewTodo>, conn: database::DbConn) -> Json<Value> {
    let todo = todo_data.into_inner();
    Todo::insert(todo.description, &conn);

    Json(json!({
        "status": "success",
    }))
}
