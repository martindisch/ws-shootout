use dotenv::dotenv;
use futures_util::{future, stream::StreamExt};
use std::env;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let server = env::var("SERVER").expect("SERVER is not set");

    let subscriber_count = 10_000;
    let address = format!("ws://{server}");

    let join_handles: Vec<_> = (0..subscriber_count)
        .into_iter()
        .map(|_| {
            let address = address.clone();
            tokio::spawn(async move { subscribe(&address).await })
        })
        .collect();

    future::join_all(join_handles).await;
}

async fn subscribe(address: &str) {
    let (mut socket, _) =
        tokio_tungstenite::connect_async(address).await.unwrap();

    while let Some(Ok(Message::Text(_))) = socket.next().await {}
}
