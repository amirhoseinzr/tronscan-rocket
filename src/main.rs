mod solana_checker;

#[macro_use] extern crate rocket;

use rocket::http::*;
use crate::solana_checker::solana_checker::solana_check;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


// ------------------------------------------------------
// blockchains : Bitcoin - Ethereum - Tron - Solana - TON


#[get("/address/<wallet_address>")]
fn specifire(wallet_address: String) -> &'static str {
        if wallet_address.starts_with("0x")
        {
             return "Ethereum blockchain";
        }

        else if wallet_address.starts_with("T") && wallet_address.len() == 34
        {
             return "Tron blockchain";
        }

        else if wallet_address.starts_with("bc1") || wallet_address.starts_with("a1") || wallet_address.starts_with("a1") && wallet_address.len() == 34
        {
             return "Bitcoin blockchain";
        }

        else if wallet_address.len() == 44
        {
            return "Solana blockchain";
        }

        else if wallet_address.starts_with("EQD")
        {
            return "TON blockchain"
        }

        else {
            return "undefined";
        //return "Unknown blockchain";
    }
}
#[get("/ad>")]
fn sol(){
    let a = solana_checker::solana_checker::solana_check();
    println!("{:?}" , &a);
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        specifire,
        sol
    ])



}