use std::sync::{Arc, Mutex};
use anyhow::Result;
use tokio::net::TcpListener;
use crate::client;
use crate::database::Database;

pub async fn start(database: Arc<Mutex<Database>>) -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("TCP Server Started");
    println!("Listening on 127.0.0.1:8080");

    loop {
        let (socket, address) = listener.accept().await?;
        println!("Client Connected : {}", address);
        let db = database.clone();
        tokio::spawn(async move {
            client::handle_client(socket, db).await;
            println!("Client Disconnected : {}", address);
        });
    }
}