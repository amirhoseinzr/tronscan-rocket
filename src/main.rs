
mod r#tron;

#[macro_use] extern crate rocket;

use std::fmt::format;
use reqwest::Client;
use rocket::response::status;
use rocket::response::status::NoContent;
use rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::tokio::time::Duration;
use rocket::{get, routes};
use serde::{Serialize, Deserialize};
use serde_json::error::Category::Data;
use rocket::config::*;

//#[derive(Serialize, Deserialize, Debug)]
// pub struct TokenInfo {
//     pub token_price_in_usd: String,
//     pub frozen_token_value_in_usd: Option<String>,
//     pub frozen: Option<i32>,
//     pub token_value: String,
//     pub token_type: i32,
//     pub token_price: String,
//     pub token_decimal: i32,
//     pub token_value_in_usd: String,
//     pub token_id: String,
//     pub token_abbr: String,
//     pub balance: String,
//     pub token_name: String,
//     pub pair_id: Option<i32>,
//     pub vip: bool,
//     pub token_url: String,
//     // Fields that may not be present in all responses
//     pub level: Option<String>,
//     pub transferCount: Option<i32>,
//     pub nrOfTokenHolders: Option<i32>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ApiResponse {
//     pub data: Vec<TokenInfo>,
//     pub count: i32,
// }

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

#[get("/token-info")]
async fn receive_wallet_info() -> Result<String , Status> {
    let wallet_address = "TEKzuPk7mS1bjG4tV82HGzHxxN26u4a2cN";
    let api_url = "https://apilist.tronscanapi.com/api/account/wallet?address=TEKzuPk7mS1bjG4tV82HGzHxxN26u4a2cN&asset_type=0";
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

#[get("/transaction_list")]
async fn transaction_list_collctor() -> Result<String, Status> {
    let api_url = "https://apilist.tronscanapi.com/api/transaction?sort=-timestamp&count=true&limit=20&start=0&start_timestamp=1529856000000&end_timestamp=1680503191391";
    let api_key = "191b7eff-354f-426a-b48f-f22331909989";
    let client = Client::new();

    let response = client
        .get(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
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

#[get("/txlist_certainaddress_trc20")]
async fn txlist_trc() -> Result<String, Status> {
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
        else{
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

    //let port = 8080;

    let config = rocket::Config::default();

    let _= rocket::build()
        .configure(config)
        .mount("/" , routes![
            welcome_page,
            wallet_info,
            wallet_modify,
            receive_wallet_trc20_transaction,
            receive_wallet_info,
            transaction_list_collctor,
            txlist_trc,
            update_wallet_info,
            delete_wallet_info
        ])
        .launch()
        .await;
}

