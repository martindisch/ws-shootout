use dotenv::dotenv;
use eventsource_client as es;
use futures_util::{
    future,
    stream::{Stream, TryStreamExt},
};
use rand::{thread_rng, Rng};
use std::{env, time::Duration};
use tokio::time;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let server = env::var("SERVER").expect("SERVER is not set");

    let subscriber_count = 10_000;
    let address = format!("http://{server}");

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

    let client = es::ClientBuilder::for_url(address)
        .unwrap()
        // .reconnect(
        //     es::ReconnectOptions::reconnect(true)
        //         .retry_initial(false)
        //         .delay(Duration::from_secs(1))
        //         .backoff_factor(2)
        //         .delay_max(Duration::from_secs(60))
        //         .build(),
        // )
        .build();

    let mut stream = tail_events(client);

    while let Ok(Some(_)) = stream.try_next().await {}
}

fn tail_events(client: impl es::Client) -> impl Stream<Item = Result<(), ()>> {
    client
        .stream()
        .map_ok(|_| ())
        .map_err(|err| eprintln!("error streaming events: {:?}", err))
}
