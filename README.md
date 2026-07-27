# 🪙 RustyWallet

A simple personal finance manager built in Rust, with a TCP client-server architecture, UDP heartbeat for session management, and optional AI-powered transaction categorization via OpenRouter.

## Features

- User registration & login (in-memory)
- Set monthly budget
- Add transactions with automatic category suggestion (AI)
- View current balance (remaining budget)
- Transaction history with timestamps
- Session management with UDP heartbeat to prevent timeouts
- Custom command-line client (alternative to telnet)

## Prerequisites

- Rust (edition 2024)
- An [OpenRouter](https://openrouter.ai/) API key (for AI categorization)

## Setup

1. Clone the repository or create the project.
2. Create a `.env` file in the project root: OPENROUTER_API_KEY=your_api_key_here
3. Build the project:
```bash
cargo build
```

## Running the server

1. Start the server:
```bash
cargo run --bin server
```
The server starts on 127.0.0.1:8080 (TCP) and listens for UDP heartbeats on 127.0.0.1:8082.

## Connecting a Client
1. Custom CLI Client (recommended)
```bash
cargo run --bin wallet_client
```
This client automatically sends UDP heartbeats (with your username after login) and properly handles multi-line history output.

2. Telnet / Netcat
```bash
telnet localhost 8080
```
Note: When using telnet, the session will expire after 30 seconds unless you manually send a UDP heartbeat containing the username (see below). History output will also appear messy due to buffering differences.

# Sending a Manual Heartbeat (for telnet users)
After logging in, you need to send a UDP packet to keep the session alive:
```bash
echo -n "ping:your_username" | nc -u 127.0.0.1 8082
```
Replace your_username with the name you used to log in.

# Stopping the Server (for telnet users)
Press `Ctrl+C` in the server terminal, or `Ctrl+]` followed by quit in telnet.

## How the Heartbeat Works
- The client (or a manual UDP packet) sends a `ping:username` message every 5 seconds to `127.0.0.1:8082`.
- The server uses the username to find the corresponding TCP session and renews its timeout.
- Without a heartbeat, the session expires after 30 seconds of inactivity (configurable in `main.rs`).

## Available Commands

- Register a new user: `register <username> <password>`
- Login: `login <username> <password>`
- logout: `logout`
- Set budget: `budget <amount>`
- Add transaction: `add <amount> <description>`
- View remaining budget: `balance`
- View all transactions (with timestamps): `history`
- Exit: `exit` or `quit`

## License
