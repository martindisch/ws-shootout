use glommio::{net::TcpListener, LocalExecutorPoolBuilder, PoolPlacement};

fn main() {
    let handles = LocalExecutorPoolBuilder::new(PoolPlacement::MaxSpread(
        num_cpus::get_physical(),
        None,
    ))
    .on_all_shards(listen)
    .unwrap();

    handles.join_all();
}

async fn listen() {
    let id = glommio::executor().id();
    println!("Executor {id} starting");

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    while let Ok(stream) = listener.accept().await {
        println!("Accepted new stream from executor {id}");
    }
}
