use futures_lite::AsyncWriteExt;
use glommio::{
    executor,
    net::{TcpListener, TcpStream},
    timer, LocalExecutorPoolBuilder, PoolPlacement,
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

    let listen_task = glommio::spawn_local(listen(listen_copy));
    let push_task = glommio::spawn_local(push(push_copy));

    listen_task.await;
    push_task.await;
}

async fn listen(streams: Rc<RefCell<Vec<Option<TcpStream>>>>) {
    let id = executor().id();

    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    while let Ok(stream) = listener.accept().await {
        let mut streams = streams.borrow_mut();
        streams.push(Some(stream));
        println!("Accepted new stream from executor {id}");
    }
}

async fn push(streams: Rc<RefCell<Vec<Option<TcpStream>>>>) {
    loop {
        {
            let mut streams = streams.borrow_mut();

            for i in 0..streams.len() {
                if let Some(stream) = &mut streams[i] {
                    if stream
                        .write_all("Hello, world!\n".as_bytes())
                        .await
                        .is_err()
                    {
                        streams[i] = None;
                    }
                }
            }
        }

        timer::sleep(Duration::from_secs(1)).await;
    }
}
