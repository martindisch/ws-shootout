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
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    let stream_count = Rc::new(RefCell::new(0));

    glommio::spawn_local(counter(Rc::clone(&stream_count))).detach();

    while let Ok(stream) = listener.accept().await {
        *stream_count.borrow_mut() += 1;
        glommio::spawn_local(handle(stream, Rc::clone(&stream_count)))
            .detach();
    }
}

async fn handle(mut stream: TcpStream, stream_count: Rc<RefCell<usize>>) {
    loop {
        if stream
            .write_all("Hello, world!\n".as_bytes())
            .await
            .is_err()
        {
            *stream_count.borrow_mut() -= 1;
            return;
        }

        timer::sleep(Duration::from_secs(1)).await;
    }
}

async fn counter(stream_count: Rc<RefCell<usize>>) {
    loop {
        println!(
            "Executor {} sent to {} clients",
            executor().id(),
            stream_count.as_ref().borrow()
        );

        timer::sleep(Duration::from_secs(1)).await;
    }
}
