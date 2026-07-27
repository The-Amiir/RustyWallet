use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::UdpSocket;
use crate::database::Database;

pub async fn start_heartbeat(database: Arc<Mutex<Database>>) -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8082").await?;
    println!("UDP Heartbeat listening on 127.0.0.1:8082");
    let mut buf = [0; 1024];
    loop {
        let (len, peer) = socket.recv_from(&mut buf).await?;
        let msg = String::from_utf8_lossy(&buf[..len]);
        let mut db = database.lock().unwrap();

        // اگر heartbeat شامل username باشد، نشست واقعی TCP را پیدا و تمدید کن
        if let Some(username) = msg.strip_prefix("ping:") {
            let address = {
                let session = db.get_session_by_username(username);
                session.map(|s| s.address)
            };

            if let Some(address) = address {
                db.update_heartbeat(address);
            }
        }
        // در غیر این صورت، heartbeat ساده (بدون username) را با همان آدرس UDP تمدید کن
        // (برای حفظ سازگاری احتمالی، ولی تأثیری بر نشست TCP ندارد)
        else {
            db.update_heartbeat(peer);
        }
    }
}
pub async fn cleanup_sessions(database: Arc<Mutex<Database>>, timeout_secs: u64) {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        let mut db = database.lock().unwrap();
        db.cleanup_sessions(timeout_secs);
    }
}