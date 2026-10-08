use concurrency::threads;

fn main() {
    threads::demo_spawn();
    threads::demo_join_after();
    threads::demo_join_before();
    threads::demo_move_closures();
}
