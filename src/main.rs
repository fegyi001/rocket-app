#[macro_use]
extern crate rocket;

use rocket::{
    response::status,
    serde::json::{json, Value},
};

#[get("/")]
fn hello() -> Value {
    json!({ "message": "Hello, world!" })
}

#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([
        {
            "id": 1,
            "name": "John Doe",
        },
        {
            "id": 2,
            "name": "Jane Doe",
        },
    ])
}

#[get("/rustaceans/<id>")]
fn get_rustacean(id: u32) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "email": "johndoe@mail.com"
    })
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!({
        "id": 1,
        "name": "John Doe",
        "email": "johndoe@mail.com"
    })
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Value {
    json!({
        "id": id,
        "name": "John Doe",
        "email": "johndoe@mail.com"
    })
}

#[delete("/rustaceans/<id>")]
fn delete_rustacean(id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                hello,
                get_rustaceans,
                get_rustacean,
                create_rustacean,
                update_rustacean,
                delete_rustacean
            ],
        )
        .launch()
        .await;
}
