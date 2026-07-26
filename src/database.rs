use std::collections::HashMap;
use std::net::SocketAddr;

use crate::models::{Session, Transaction, User};

pub struct Database {
    pub users: HashMap<String, User>,
    pub sessions: HashMap<SocketAddr, Session>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        username: String,
        password: String,
    ) -> bool {
        if self.users.contains_key(&username) {
            return false;
        }

        let user = User::new(username.clone(), password);
        self.users.insert(username, user);
        true
    }

    pub fn login(
        &mut self,
        username: String,
        password: String,
        address: SocketAddr,
    ) -> bool {

        let user = match self.users.get(&username) {
            Some(user) => user,
            None => return false,
        };

        if user.password != password {
            return false;
        }

        let session = Session::new(username, address);
        self.sessions.insert(address, session);
        true
    }

    pub fn logout(&mut self, address: SocketAddr) {
        self.sessions.remove(&address);
    }

    pub fn get_username(
        &self,
        address: SocketAddr,
    ) -> Option<String> {

        self.sessions
            .get(&address)
            .map(|s| s.username.clone())
    }

    pub fn add_transaction(
        &mut self,
        username: &str,
        transaction: Transaction,
    ) {

        if let Some(user) = self.users.get_mut(username) {
            user.balance += transaction.amount;
            user.transactions.push(transaction);
        }

    }

    pub fn get_balance(
        &self,
        username: &str,
    ) -> Option<f64> {

        self.users.get(username).map(|u| u.balance)
    }

    pub fn get_transactions(
        &self,
        username: &str,
    ) -> Option<Vec<Transaction>> {

        self.users
            .get(username)
            .map(|u| u.transactions.clone())

    }
}