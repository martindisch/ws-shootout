use futures_util::stream::StreamExt;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let (mut socket, _) = tokio_tungstenite::connect_async("ws://127.0.0.1:8080")
        .await
        .unwrap();

    while let Some(Ok(Message::Text(text))) = socket.next().await {
        println!("{}", text);
    }
}
