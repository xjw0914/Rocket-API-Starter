#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    format!("Hello, {}", name)
}
