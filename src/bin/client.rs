use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=========================");
    println!("  RustyWallet Client");
    println!("=========================");
    println!("Connecting to server...");

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connected to server!");
    println!("Type your commands (or 'exit' to quit)");
    println!("----------------------------------------");

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" || input == "quit" {
            println!("Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

       
        let cmd = format!("{}\n", input);
        if let Err(e) = writer.write_all(cmd.as_bytes()).await {
            println!("Error sending command: {}", e);
            break;
        }

       
        let mut response = String::new();
        match reader.read_line(&mut response).await {
            Ok(0) => {
                println!("Server disconnected");
                break;
            }
            Ok(_) => {
                print!("{}", response);
            }
            Err(e) => {
                println!("Error reading response: {}", e);
                break;
            }
        }
    }

    Ok(())
}