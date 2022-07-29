use dotenv::dotenv;
use eventsource_client as es;
use futures_util::{
    future,
    stream::{Stream, TryStreamExt},
};
use hyper::client::{Client, HttpConnector};
use rand::{thread_rng, Rng};
use std::{env, time::Duration};
use tokio::time;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let server = env::var("SERVER").expect("SERVER is not set");

    let subscriber_count = 60_000;
    let address = format!("http://{server}");

    let shared_client = Client::new();

    let join_handles: Vec<_> = (0..subscriber_count)
        .into_iter()
        .map(|_| {
            let client = shared_client.clone();
            let address = address.clone();
            tokio::spawn(async move { subscribe(client, &address).await })
        })
        .collect();

    future::join_all(join_handles).await;
}

async fn subscribe(client: Client<HttpConnector>, address: &str) {
    // Wait for some amount of time to prevent everybody connecting all at once
    let delay_secs = thread_rng().gen_range(1..=30);
    time::sleep(Duration::from_secs(delay_secs)).await;

    let client = es::ClientBuilder::for_url(address)
        .unwrap()
        .build_with_http_client(client);

    let mut stream = tail_events(client);

    while let Ok(Some(_)) = stream.try_next().await {}
}

fn tail_events(client: impl es::Client) -> impl Stream<Item = Result<(), ()>> {
    client
        .stream()
        .map_ok(|_| ())
        .map_err(|err| eprintln!("error streaming events: {:?}", err))
}
