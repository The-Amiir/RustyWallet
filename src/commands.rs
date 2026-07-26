use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::task::spawn_blocking;
use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::Transaction;
use crate::ai::categorize;

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
        "budget" => {
            if args.len() != 1 {
                return Err(AppError::InvalidCommand);
            }
            let amount: f64 = args[0]
                .parse()
                .map_err(|_| AppError::InvalidAmount)?;
            
            let username = {
                let db = database.lock().unwrap();
                db.get_username(address)
                    .ok_or(AppError::NotLoggedIn)?
            };

            let mut db = database.lock().unwrap();
            if db.set_budget(&username, amount) {
                Ok(format!("Budget set to: {}", amount))
            } else {
                Err(AppError::Internal("Failed to set budget".to_string()))
            }
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

            
            {
                let db = database.lock().unwrap();
                if let Some(budget) = db.get_budget(&username) {
                    if budget < amount {
                        return Err(AppError::Internal(format!(
                            "Insufficient budget. Remaining: {}", 
                            budget
                        )));
                    }
                }
            }

            let category = categorize(&description).await?;

            let transaction = Transaction {
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
            
            // Get remaining budget
            let remaining = db.get_budget(&username).unwrap_or(0.0);
            
            Ok(format!("Added: {} ({}) - Remaining budget: {}", amount, category, remaining))
        }
        "balance" => {
            let db = database.lock().unwrap();
            let username = db.get_username(address)
                .ok_or(AppError::NotLoggedIn)?;
            let budget = db.get_budget(&username)
                .unwrap_or(0.0);
            Ok(format!("Remaining budget: {}", budget))
        }
        "history" => {
            let username = {
                let db = database.lock().unwrap();
                db.get_username(address)
                    .ok_or(AppError::NotLoggedIn)?
            };

            let transactions = {
                let db = database.lock().unwrap();
                db.get_transactions(&username)
                    .unwrap_or_else(Vec::new)
            };

            let result = spawn_blocking(move || {
                if transactions.is_empty() {
                    return "No transactions".to_string();
                }
                let mut output = String::from("History:\n");
                for tx in transactions {
                    output.push_str(&format!(
                        "{} - {}: {} ({})\n",
                        tx.timestamp, tx.category, tx.amount, tx.description
                    ));
                }
                output
            })
            .await
            .map_err(|e| AppError::Internal(format!("spawn_blocking: {}", e)))?;

            Ok(result)
        }
        _ => Err(AppError::InvalidCommand),
    }
}