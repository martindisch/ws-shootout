use chrono::offset::Local;
use futures_util::SinkExt;
use std::time::Duration;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
    time,
};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(8);
    tokio::spawn(publish(tx.clone()));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = tokio_tungstenite::accept_async(stream).await.unwrap();
        tokio::spawn(handle(ws_stream, tx.subscribe()));
    }
}

async fn publish(tx: Sender<String>) {
    loop {
        let now = Local::now();
        let time = now.time().format("%H:%M:%S");
        match tx.send(now.to_string()) {
            Ok(cnt) => {
                println!("[{time}] Sending date to {cnt} clients")
            }
            Err(_) => println!("[{time}] Sending date to 0 clients",),
        }

        time::sleep(Duration::from_secs(1)).await;
    }
}

async fn handle(
    mut stream: WebSocketStream<TcpStream>,
    mut rx: Receiver<String>,
) {
    while let Ok(message) = rx.recv().await {
        if stream.send(Message::Text(message)).await.is_err() {
            return;
        }
    }
}
