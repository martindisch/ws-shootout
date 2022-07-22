use dotenv::dotenv;
use futures_util::future;
use rand::{thread_rng, Rng};
use std::{env, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpStream,
    time,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let server = env::var("SERVER").expect("SERVER is not set");

    let subscriber_count = 10_000;

    let join_handles: Vec<_> = (0..subscriber_count)
        .into_iter()
        .map(|_| {
            let address = server.clone();
            tokio::spawn(async move { subscribe(&address).await })
        })
        .collect();

    future::join_all(join_handles).await;
}

async fn subscribe(address: &str) {
    // Wait for some amount of time to prevent everybody connecting all at once
    let delay_secs = thread_rng().gen_range(1..=30);
    time::sleep(Duration::from_secs(delay_secs)).await;

    let stream = TcpStream::connect(address).await.unwrap();
    let mut reader = BufReader::new(stream);
    let mut buf = String::with_capacity(40);

    while let Ok(n) = reader.read_line(&mut buf).await {
        if n == 0 {
            return;
        }

        buf.clear();
    }
}
