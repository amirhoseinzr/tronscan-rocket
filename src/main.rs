
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
use csv::Writer;
use std::fs::File;
use std::io::Write;
use rocket::futures::StreamExt;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;



//------------------------------------ IO Layer   ----------------------------------------------//


//------------------------------------ ********   ----------------------------------------------//





//------------------------------------    ----------------------------------------------//
#[derive(Debug)]
struct Wallet {
    public_key: String,

}




#[derive(Deserialize)]
struct WalletRelatedAddresses {
    wallet_address: Wallet,
    relate_addresses: Vec<Wallet>
}










//---------------------- **************** ---------------------------------//

// ----------------------  INTERACT LAYER -----------------------------//
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

//---------------------- **************** ---------------------------------//


//-------------   get to receive wallet trc20 transactions   ----------------\\

#[get("/wallet_trx")]
async fn receive_wallet_trc20_transaction() -> Result<String , Status> {
    let api_url = "https://apilist.tronscanapi.com/api/transfer/\
    trc20?address=YwFDp7SBfZk7Hrz3tucwQVASyJdwC7\
    &trc20Id=TCmSR8UYWvsZkZmprGKaudTuWUZ62ycnnN&start\
    =0&limit=2&direction=0&reverse=true&db_version=1&start_timestamp=&end_timestamp=";
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
        let data_out = data.clone();
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







//------------------------------------------------------------------------

#[rocket::main]
async fn main() -> surrealdb::Result<> {
    let db = Surreal::new::<Ws>("localhost:8001").await?;

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

