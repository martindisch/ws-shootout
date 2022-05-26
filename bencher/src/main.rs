use dotenv::dotenv;
use futures_util::{future, stream::StreamExt};
use rand::{thread_rng, Rng};
use std::{env, time::Duration};
use tokio::time;
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
    // Wait for some amount of time to prevent everybody connecting all at once
    let delay_secs = thread_rng().gen_range(1..=30);
    time::sleep(Duration::from_secs(delay_secs)).await;

    let (mut socket, _) =
        tokio_tungstenite::connect_async(address).await.unwrap();

    while let Some(Ok(Message::Text(_))) = socket.next().await {}
}
