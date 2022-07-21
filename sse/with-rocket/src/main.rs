use chrono::offset::Local;
use rocket::{
    response::stream::{Event, EventStream},
    tokio::{
        self,
        sync::broadcast::{self, Sender},
        time,
    },
    State,
};
use std::time::Duration;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let (tx, _) = broadcast::channel(8);
    tokio::spawn(publish(tx.clone()));

    let _rocket = rocket::build()
        .manage(tx)
        .mount("/", routes![stream])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
async fn stream(tx: &State<Sender<String>>) -> EventStream![] {
    let mut rx = tx.subscribe();

    EventStream! {
        loop {
            while let Ok(message) = rx.recv().await {
                yield Event::data(message);
            }
        }
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
