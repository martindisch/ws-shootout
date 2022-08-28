use glommio::{
    net::{TcpListener, TcpStream},
    LocalExecutorPoolBuilder, PoolPlacement,
};

fn main() {
    let handles = LocalExecutorPoolBuilder::new(PoolPlacement::MaxSpread(
        num_cpus::get_physical(),
        None,
    ))
    .on_all_shards(serve)
    .unwrap();

    handles.join_all();
}

async fn serve() {
    let id = glommio::executor().id();
    println!("Executor {id} starting");

    let mut streams = Vec::new();

    listen(&mut streams).await;
}

async fn listen(streams: &mut Vec<TcpStream>) {
    let id = glommio::executor().id();

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    while let Ok(stream) = listener.accept().await {
        streams.push(stream);
        println!("Accepted new stream from executor {id}");
    }
}
