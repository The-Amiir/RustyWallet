use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use crate::database::Database;
use crate::errors::{AppError, AppResult};

pub async fn handle_command(
    database: Arc<Mutex<Database>>,
    address: SocketAddr,
    line: &str,
) -> AppResult<String> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Err(AppError::InvalidCommand);
    }

    let command = parts[0].to_lowercase();
    let args = &parts[1..];

    match command.as_str() {
        "register" => {
            if args.len() != 2 {
                return Err(AppError::InvalidCommand);
            }
            let username = args[0].to_string();
            let password = args[1].to_string();
            let mut db = database.lock().unwrap();
            if db.register(username, password) {
                Ok("Registered successfully".to_string())
            } else {
                Err(AppError::UserExists)
            }
        }
        "login" => {
            if args.len() != 2 {
                return Err(AppError::InvalidCommand);
            }
            let username = args[0].to_string();
            let password = args[1].to_string();
            let mut db = database.lock().unwrap();
            if db.login(username, password, address) {
                Ok("Logged in".to_string())
            } else {
                Err(AppError::InvalidCredentials)
            }
        }
        "logout" => {
            let mut db = database.lock().unwrap();
            db.logout(address);
            Ok("Logged out".to_string())
        }
        "balance" => {
            let db = database.lock().unwrap();
            let username = db.get_username(address)
                .ok_or(AppError::NotLoggedIn)?;
            let balance = db.get_balance(&username)
                .unwrap_or(0.0);
            Ok(format!("Balance: {}", balance))
        }
        "add" => {
            if args.len() < 2 {
                return Err(AppError::InvalidCommand);
            }
            let amount: f64 = args[0]
                .parse()
                .map_err(|_| AppError::InvalidAmount)?;
            let description = args[1..].join(" ");

            let username = {
                let db = database.lock().unwrap();
                db.get_username(address)
                    .ok_or(AppError::NotLoggedIn)?
            };

            let category = crate::ai::categorize(&description).await?;

            let transaction = crate::models::Transaction {
                amount,
                description: description.clone(),
                category: category.clone(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            let mut db = database.lock().unwrap();
            db.add_transaction(&username, transaction);
            Ok(format!("Added: {} ({})", amount, category))
        }
        _ => Err(AppError::InvalidCommand),
    }
}