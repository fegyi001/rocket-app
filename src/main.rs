#[macro_use]
extern crate rocket;

use rocket::{
    fairing::{Fairing, Info, Kind},
    response::status,
    serde::json::{json, Value},
    Request, Response,
};

pub struct RemoveServerHeader;

#[rocket::async_trait]
impl Fairing for RemoveServerHeader {
    fn info(&self) -> Info {
        Info {
            name: "Remove Server Header",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        // Remove the 'Server' header
        res.remove_header("Server");
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!({ "message": "Not Found" })
}

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

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .attach(RemoveServerHeader)
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
        .register("/", catchers![not_found])
        .launch()
        .await;
}
