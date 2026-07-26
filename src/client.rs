use std::sync::{Arc, Mutex};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::database::Database;

pub async fn handle_client(
    stream: TcpStream,
    _database: Arc<Mutex<Database>>,
) {

    let address = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(_) => return,
    };

    println!("Handling client: {}", address);

    let (reader, mut writer) = stream.into_split();

    let mut reader = BufReader::new(reader);

    if writer
        .write_all(b"Welcome to Rusty Wallet Server\n")
        .await
        .is_err()
    {
        return;
    }

    if writer
        .write_all(b"Type 'exit' to disconnect.\n")
        .await
        .is_err()
    {
        return;
    }

    let mut line = String::new();

    loop {

        line.clear();

        let bytes = match reader.read_line(&mut line).await {
            Ok(size) => size,
            Err(_) => break,
        };

        if bytes == 0 {
            break;
        }

        let command = line.trim();
        if command.eq_ignore_ascii_case("exit") {

            if writer
                .write_all(b"Goodbye!\n")
                .await
                .is_err()
            {
                break;
            }

            break;
        }

        let response = format!(
            "Server received: {}\n",
            command
        );

        if writer
            .write_all(response.as_bytes())
            .await
            .is_err()
        {
            break;
        }
    }

}