use chrono::offset::Local;
use futures_util::StreamExt;
use std::convert::Infallible;
use std::time::Duration;
use tokio::{
    sync::broadcast::{self, Sender},
    time,
};
use tokio_stream::wrappers::BroadcastStream;
use warp::{sse::Event, Filter};

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(8);
    tokio::spawn(publish(tx.clone()));

    let routes = warp::path::end().and(warp::get()).map(move || {
        let stream = BroadcastStream::new(tx.subscribe());
        let event_stream = stream.map(|time| sse_time(time.unwrap()));
        warp::sse::reply(event_stream)
    });

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
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

fn sse_time(time: String) -> Result<Event, Infallible> {
    Ok(Event::default().data(time))
}
