#[macro_use] extern crate rocket;


use std::fmt::format;
use reqwest::Client;
use rocket::response::status;
use rocket::response::status::NoContent;
use rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::tokio::time::Duration;
use rocket::{get, routes};
use serde_json::error::Category::Data;

#[get("/")]
fn welcome_page() -> Value {
    json!("tronscann-rocket")
}

#[get("/wallet/<id>")]
fn wallet_info(id: &str) -> Value {
    json!({"id:":id , "public_key": "public-key example" })
}

#[post("/wallet" , format = "json")]
fn wallet_modify() -> Value {
    json!({"public_key": "public-key example" })
}

//-------------   get to receive wallet trc20 transactions   ----------------\\

#[get("/wallet_trx")]
async fn receive_wallet_trc20_transaction() -> Result<String , Status> {
    let api_url = "https://apilist.tronscanapi.com/api/transfer/trc20?address=TSTVYwFDp7SBfZk7Hrz3tucwQVASyJdwC7&trc20Id=TCmSR8UYWvsZkZmprGKaudTuWUZ62ycnnN&start=0&limit=2&direction=0&reverse=true&db_version=1&start_timestamp=&end_timestamp=";
    let api_key = "191b7eff-354f-426a-b48f-f22331909989";

    let client = Client::new();

    let response = client
        .get(api_url)
        .header("Authorization" , format!("Bearer {}" , api_key))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?;

    if response.status().is_success() {
        let data = response.text().await.map_err(|_| Status::InternalServerError)?;
        Ok(data)
    }
        else {
            Err(Status::InternalServerError)
        }

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
            receive_wallet_trc20_transaction,
            update_wallet_info,
            delete_wallet_info
        ])
        .launch()
        .await;
}
