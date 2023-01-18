use tokio::runtime::Builder;

use gameserver::server::app;

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("web-framework-testing")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .unwrap();
    runtime.block_on(app::start_app());
}
