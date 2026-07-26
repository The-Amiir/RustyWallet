mod ai;
mod client;
mod commands;
mod database;
mod errors;
mod models;
mod server;
mod udp;

use database::Database;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {

    println!("========================");
    println!(" Rusty Wallet Server");
    println!("========================");

    let database = Arc::new(Mutex::new(Database::new()));

    if let Err(error) = server::start(database).await {
        println!("Server Error : {}", error);
    }

}