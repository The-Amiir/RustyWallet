use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub amount: f64,
    pub description: String,
    pub category: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub balance: f64,
    pub transactions: Vec<Transaction>,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            balance: 0.0,
            transactions: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Session {
    pub username: String,
    pub address: SocketAddr,
}

impl Session {
    pub fn new(username: String, address: SocketAddr) -> Self {
        Self {
            username,
            address,
        }
    }
}