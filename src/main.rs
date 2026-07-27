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
    dotenvy::dotenv().ok();

    println!("========================");
    println!(" Rusty Wallet Server");
    println!("========================");

    let database = Arc::new(Mutex::new(Database::new()));

    let db_tcp = database.clone();
    let db_udp = database.clone();
    let db_cleanup = database.clone();

    let tcp_handle = tokio::spawn(async move {
        if let Err(error) = server::start(db_tcp).await {
            println!("Server Error : {}", error);
        }
    });

    let udp_handle = tokio::spawn(async move {
        if let Err(error) = udp::start_heartbeat(db_udp).await {
            println!("UDP Error : {}", error);
        }
    });

    let cleanup_handle = tokio::spawn(async move {
        udp::cleanup_sessions(db_cleanup, 30).await;
    });

    tokio::select! {
        _ = tcp_handle => {},
        _ = udp_handle => {},
        _ = cleanup_handle => {},
    }
}