use concurrency::{channels, shared_state, threads};

fn main() {
    threads::demo_spawn();
    threads::demo_join_after();
    threads::demo_join_before();
    threads::demo_move_closures();

    channels::demo_channel();
    channels::demo_channel_stream();
    channels::demo_multiple_producers();

    shared_state::demo_mutex_api();
}
