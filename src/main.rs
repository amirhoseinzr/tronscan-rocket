
#[macro_use] extern crate rocket;

use std::fmt::format;
use reqwest::Client;
use rocket::response::status::NoContent;
use rocket::serde::json::{json, Json};
use rocket::http::{Method, Status};
use rocket::tokio::time::Duration;
use rocket::{get, routes};
use serde::{Serialize, Deserialize};
use serde_json::error::Category::Data;
use rocket::config::*;
use rocket::State;
use rocket::response::content::RawJson;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use rocket_cors::CorsOptions;
use mysql_async::{Pool, prelude::Queryable};
use serde_json::{value, Value};
use crate::rocket::yansi::Paint;

// use mysql::Pool;


//#[get("/")]
// fn welcome_page() -> Value {
//     json!("tronscann-rocket")
// }
//
// #[get("/wallet/<id>")]
// fn wallet_info(id: &str) -> Value {
//     json!({"id:":id , "public_key": "public-key example" })
// }
//
// #[post("/wallet" , format = "json")]
// fn wallet_modify() -> Value {
//     json!({"public_key": "public-key example" })
// }

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


#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    block_number: String,
    block_hash: String,
    time_stamp: String,
    tx_hash: String,
    from: String,
    to: String,
    value: String,
    gas: String,
    gas_price: String,
    input: String,
    function_name: String,
    confirmations: String,
}
#[get("/binance/txlist/<address>")]
//async fn bscscan_txlist(address: &str) -> Result<RawJson<String> , Status>{
async fn bscscan_txlist(address: &str, pool: &State<Pool>) -> Result<Json<Vec<Transaction>> , Status>{
    let api_key = "2B9TDS2QM2WIJSQREAJU9V6K5ZXB9Q7CBW";
    let aapi_url = format!("https://api.bscscan.com/api\
   ?module=account\
   &action=txlist\
   &address={}\
   &startblock=0\
   &endblock=99999999\
   &page=1\
   &offset=10\
   &sort=asc\
   &apikey={}" , address,api_key);

    let client = Client::new();

    let response = client
            .get(&aapi_url)
        .timeout(Duration::from_secs(1000))
        .send()
        .await
        .map_err(|_| Status::InternalServerError)?;



    if response.status().is_success() {
        //let data:String = response.text().await.map_err(|_| Status::InternalServerError)?;
        let data: Value = response
            .json()
            .await
            .map_err(|_|Status::InternalServerError)?;

        let transactions: Vec<Transaction> = data["result"]
            .as_array()
            .unwrap()
            .iter()
            .map(|tx| Transaction {
                block_number: tx["blockNumber"].as_str().unwrap().to_string(),
                block_hash: tx["blockHash"].as_str().unwrap().to_string(),
                time_stamp: tx["timeStamp"].as_str().unwrap().to_string(),
                tx_hash: tx["hash"].as_str().unwrap().to_string(),
                from: tx["from"].as_str().unwrap().to_string(),
                to: tx["to"].as_str().unwrap().to_string(),
                value: tx["value"].as_str().unwrap().to_string(),
                gas: tx["gas"].as_str().unwrap().to_string(),
                gas_price: tx["gasPrice"].as_str().unwrap().to_string(),
                input: tx["input"].as_str().unwrap().to_string(),
                function_name: tx["functionName"].as_str().unwrap().to_string(),
                confirmations: tx["confirmations"].as_str().unwrap().to_string(),
            })
            .collect();

        let mut conn = pool.get_conn().await.map_err(|_| Status::InternalServerError)?;

        for tx in &transactions {
            let query = format!(
                "INSERT INTO transactions (block_number, block_hash, time_stamp, tx_hash, `from`, `to`, value, gas, gas_price, input, function_name, confirmations)
                 VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
                tx.block_number, tx.block_hash, tx.time_stamp, tx.tx_hash, tx.from, tx.to, tx.value, tx.gas, tx.gas_price, tx.input, tx.function_name, tx.confirmations
            );

            conn.query_drop(query).await.map_err(|_| Status::InternalServerError)?;
        }
        Ok(Json(transactions))
    }
    else {
        Err(Status::InternalServerError)
    }
}

#[get("/txlist_certainaddress_trc20")]
async fn txlist_trc() -> Result<String, Status> {
    let api_url = "https://apilist.tronscanapi.com/api/transfer/\
    trc20?address=TSTVYwFDp7SBfZk7Hrz3tucwQVASyJdwC7&trc20Id\
    =TCmSR8UYWvsZkZmprGKaudTuWUZ62ycnnN&start=0&limit=100stat&direction=0&reverse=true&db_version=1&start_timestamp=&end_timestamp=";
    let api_key = "191b7eff-354f-426a-b48f-f22331909989";
    let client = Client::new();

    let response = client
        .get(api_url)
        .header("Authorization" , format!("Bearer {}" , api_key))
        .timeout(Duration::from_secs(100))
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


// #[put("/wallet/<id>" , format = "json")]
// fn update_wallet_info(id: &str) -> serde_json::value {
//     json!({"id:":id , "public_key": "public-key example" })
// }
//
// #[delete("/wallet/<id>")]
// fn delete_wallet_info(id: &str) -> status::NoContent {
//     status::NoContent
// }

#[rocket::main]
async fn main() {

    let database_url = "mysql://root:Aa@48432610@localhost/bsc";
    let pool = Pool::new(database_url);

    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),  // Allow any headers
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("Failed to create CORS config");

    //let port = 8080;

    let config = rocket::Config::default();   // add custom config to this

    let _= rocket::build()
        .attach(cors)
        .configure(config)
        .manage(pool)
        .mount("/" , routes![
            // welcome_page,
            // wallet_info,
            // wallet_modify,
            receive_wallet_trc20_transaction,
            receive_wallet_info,
            transaction_list_collctor,
            txlist_trc,
            // update_wallet_info,
            // delete_wallet_info,
            bscscan_txlist
        ])
        .launch()
        .await;
}
