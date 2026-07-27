use std::io::{self, Write};
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::{Duration, sleep};

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

    let udp_socket = Arc::new(UdpSocket::bind("0.0.0.0:0")?);
    let udp_socket_clone = udp_socket.clone();

    // اشتراک‌گذاری نام کاربری جاری بین thread اصلی و تسک heartbeat
    let current_user = Arc::new(Mutex::new(None::<String>));
    let current_user_clone = current_user.clone();

    // تسک heartbeat: هر ۵ ثانیه پیام می‌فرستد (در صورت ورود، همراه با username)
    tokio::spawn(async move {
        loop {
            let msg = {
                let user = current_user_clone.lock().unwrap();
                if let Some(ref username) = *user {
                    format!("ping:{}", username)
                } else {
                    "ping".to_string()
                }
            };
            if let Err(e) = udp_socket_clone.send_to(msg.as_bytes(), "127.0.0.1:8082") {
                eprintln!("Heartbeat error: {}", e);
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let stdin = io::stdin();

    // برای نگه‌داشتن username در هنگام login (قبل از دریافت پاسخ)
    let mut pending_login: Option<String> = None;

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim().to_string();

        if input == "exit" || input == "quit" {
            println!("Goodbye!");
            // پاکسازی username هنگام خروج
            *current_user.lock().unwrap() = None;
            break;
        }

        if input.is_empty() {
            continue;
        }

        // اگر دستور login باشد، username را ذخیره موقت کن
        if input.starts_with("login ") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() == 3 {
                pending_login = Some(parts[1].to_string());
            }
        }

        // اگر دستور logout باشد، username را پاک کن
        if input == "logout" {
            *current_user.lock().unwrap() = None;
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
                let resp = response.trim_end().to_string();
                print!("{}", response); // چاپ پاسخ

                // اگر پاسخ سرور نشان‌دهندهٔ ورود موفق باشد، username نهایی شود
                if resp == "Logged in" {
                    if let Some(user) = pending_login.take() {
                        let mut lock = current_user.lock().unwrap();
                        *lock = Some(user);
                    }
                } else {
                    // در غیر این صورت، pending_login را فراموش کن
                    pending_login = None;
                }
            }
            Err(e) => {
                println!("Error reading response: {}", e);
                break;
            }
        }
    }

    Ok(())
}