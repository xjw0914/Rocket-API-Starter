use rocket_contrib::{Json, Value};

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

#[error(500)]
fn internal_error() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Internal server error occurred."
    }))
}

#[error(400)]
fn bad_request() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Bad request was got."
    }))
}
