use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use crate::database::Database;
use crate::commands::handle_command;

pub async fn handle_client(
    mut socket: TcpStream,
    database: Arc<Mutex<Database>>,
) {
    let address = socket.peer_addr().unwrap();
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                let response = match handle_command(database.clone(), address, &line).await {
                    Ok(msg) => msg,
                    Err(e) => format!("Error: {}", e),
                };
                let output = format!("{}\n", response);
                if let Err(e) = writer.write_all(output.as_bytes()).await {
                    eprintln!("write error: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("read error: {}", e);
                break;
            }
        }
    }
}