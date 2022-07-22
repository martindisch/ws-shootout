use chrono::offset::Local;
use std::time::Duration;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
    time,
};

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(8);
    tokio::spawn(publish(tx.clone()));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle(stream, tx.subscribe()));
    }
}

async fn publish(tx: Sender<String>) {
    loop {
        let now = Local::now();
        let time = now.time().format("%H:%M:%S");
        match tx.send(format!("{now}\n")) {
            Ok(cnt) => {
                println!("[{time}] Sending date to {cnt} clients")
            }
            Err(_) => println!("[{time}] Sending date to 0 clients",),
        }

        time::sleep(Duration::from_secs(1)).await;
    }
}

async fn handle(mut stream: TcpStream, mut rx: Receiver<String>) {
    while let Ok(message) = rx.recv().await {
        if stream.write_all(message.as_bytes()).await.is_err() {
            return;
        }
    }
}
