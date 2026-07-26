# ===========================
# RUSTYWALLET - ALL COMMANDS 
# ===========================

# ---------- PROJECT SETUP ----------
cargo new rustywallet
cd rustywallet
echo "OPENAI_API_KEY=your_openai_api_key_here" > .env

# ---------- BUILD ----------
cargo build
cargo build --release

# ---------- RUN SERVER ----------
cargo run

# ---------- CONNECT WITH TELNET ----------
telnet 127.0.0.1 8080

# ---------- SEND HEARTBEAT (UDP) ----------
echo "ping" | nc -u 127.0.0.1 8081

# ---------- TELNET COMMANDS ----------
register <username> <password>
login <username> <password>
logout
budget <amount>
add <amount> <description>
balance
history

# ---------- TELNET COMMANDS (EXAMPLE) ----------
register ali 123
login ali 123
budget 10000
add 25000 lunch
add 12000 taxi
add 5000 book
balance
history
logout

# ---------- EXIT TELNET ----------
Ctrl + ]
quit

# ---------- STOP SERVER ----------
Ctrl + C
