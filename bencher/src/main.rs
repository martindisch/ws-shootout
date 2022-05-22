use futures_util::{future, stream::StreamExt};
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let subscriber_count = 30_000;
    let address = "ws://127.0.0.1:8080";

    let join_handles: Vec<_> = (0..subscriber_count)
        .into_iter()
        .map(|_| tokio::spawn(subscribe(address)))
        .collect();

    future::join_all(join_handles).await;
}

async fn subscribe(address: &str) {
    let (mut socket, _) = tokio_tungstenite::connect_async(address).await.unwrap();

    while let Some(Ok(Message::Text(_))) = socket.next().await {}
}
