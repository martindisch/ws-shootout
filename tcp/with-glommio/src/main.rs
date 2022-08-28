use glommio::{LocalExecutorPoolBuilder, PoolPlacement};

fn main() {
    let handles = LocalExecutorPoolBuilder::new(PoolPlacement::Unbound(
        num_cpus::get_physical(),
    ))
    .on_all_shards(|| async move {
        let id = glommio::executor().id();
        println!("hello from executor {}", id);
    })
    .unwrap();

    handles.join_all();
}
