use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{Session, Transaction, User};

pub struct Database {
    pub users: HashMap<String, User>,
    pub sessions: HashMap<SocketAddr, Session>,
    pub last_heartbeat: HashMap<SocketAddr, u64>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
            last_heartbeat: HashMap::new(),
        }
    }

    pub fn register(&mut self, username: String, password: String) -> bool {
        if self.users.contains_key(&username) {
            return false;
        }

        let user = User::new(password);
        self.users.insert(username, user);
        true
    }

    pub fn login(&mut self, username: String, password: String, address: SocketAddr) -> bool {
        let user = match self.users.get(&username) {
            Some(user) => user,
            None => return false,
        };

        if user.password != password {
            return false;
        }

        let session = Session::new(username, address);
        self.sessions.insert(address, session);
        self.update_heartbeat(address);
        true
    }

    pub fn logout(&mut self, address: SocketAddr) {
        self.sessions.remove(&address);
        self.last_heartbeat.remove(&address);
    }

    pub fn get_username(&self, address: SocketAddr) -> Option<String> {
        self.sessions
            .get(&address)
            .map(|s| s.username.clone())
    }

    pub fn add_transaction(&mut self, username: &str, transaction: Transaction) {
        if let Some(user) = self.users.get_mut(username) {
            user.balance += transaction.amount;
            user.budget -= transaction.amount;
            user.transactions.push(transaction);
        }
    }

    pub fn get_budget(&self, username: &str) -> Option<f64> {
        self.users.get(username).map(|u| u.budget)
    }

    pub fn set_budget(&mut self, username: &str, amount: f64) -> bool {
        if let Some(user) = self.users.get_mut(username) {
            user.budget = amount;
            true
        } else {
            false
        }
    }

    pub fn get_transactions(&self, username: &str) -> Option<Vec<Transaction>> {
        self.users
            .get(username)
            .map(|u| u.transactions.clone())
    }

    pub fn update_heartbeat(&mut self, address: SocketAddr) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.last_heartbeat.insert(address, now);
        println!("Heartbeat received from: {}", address);
    }

    pub fn get_session_by_username(&self, username: &str) -> Option<&Session> {
        self.sessions.values().find(|s| s.username == username)
    }

    pub fn cleanup_sessions(&mut self, timeout_secs: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let expired: Vec<SocketAddr> = self
            .sessions
            .keys()
            .filter(|addr| {
                if let Some(&last) = self.last_heartbeat.get(addr) {
                    now - last > timeout_secs
                } else {
                    true
                }
            })
            .cloned()
            .collect();

            if !expired.is_empty() {
                println!("[CLEANUP] Removing sessions: {:?}", expired);
            }
            for addr in expired {
                self.sessions.remove(&addr);
                self.last_heartbeat.remove(&addr);
            }
    }
}