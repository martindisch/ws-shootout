use futures_lite::{future, AsyncWriteExt};
use glommio::{
    executor,
    net::{TcpListener, TcpStream},
    timer, Latency, LocalExecutorPoolBuilder, PoolPlacement, Shares,
};
use std::{cell::RefCell, rc::Rc, time::Duration};

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
    let id = executor().id();
    println!("Executor {id} starting");

    let streams = Rc::new(RefCell::new(Vec::new()));
    let listen_copy = Rc::clone(&streams);
    let push_copy = Rc::clone(&streams);

    let listen_queue = executor().create_task_queue(
        Shares::Static(1),
        Latency::NotImportant,
        "listen",
    );
    let push_queue = executor().create_task_queue(
        Shares::Static(1),
        Latency::NotImportant,
        "push",
    );

    let listen_task =
        glommio::spawn_local_into(listen(listen_copy), listen_queue).unwrap();
    let push_task =
        glommio::spawn_local_into(push(push_copy), push_queue).unwrap();

    future::zip(listen_task, push_task).await;
}

async fn listen(streams: Rc<RefCell<Vec<TcpStream>>>) {
    let id = executor().id();

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    while let Ok(stream) = listener.accept().await {
        let mut streams = streams.borrow_mut();
        streams.push(stream);
        println!("Accepted new stream from executor {id}");
    }
}

async fn push(streams: Rc<RefCell<Vec<TcpStream>>>) {
    loop {
        {
            let mut streams = streams.borrow_mut();
            for stream in streams.iter_mut() {
                if stream
                    .write_all("Hello, world!\n".as_bytes())
                    .await
                    .is_err()
                {
                    return;
                }
            }
        }

        timer::sleep(Duration::from_secs(1)).await;
    }
}
