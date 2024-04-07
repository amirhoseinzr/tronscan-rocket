#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::response::status::NoContent;
use rocket::serde::json::{json, Value};

#[get("/")]
fn welcome_page() -> Value {
    json!("Welcome Page")
}

#[get("/wallet/<id>")]
fn wallet_info(id: &str) -> Value {
    json!({"id:":id , "public_key": "public-key example" })
}

#[post("/wallet" , format = "json")]
fn wallet_modify() -> Value {
    json!({"public_key": "public-key example" })
}

#[put("/wallet/<id>" , format = "json")]
fn update_wallet_info(id: &str) -> Value {
    json!({"id:":id , "public_key": "public-key example" })
}

#[delete("/wallet/<id>")]
fn delete_wallet_info(id: &str) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/" , routes![
            welcome_page,
            wallet_info,
            wallet_modify,
            update_wallet_info,
            delete_wallet_info
        ])
        .launch()
        .await;
}
