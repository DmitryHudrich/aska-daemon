#[macro_use]
extern crate log;
extern crate env_logger;

mod server;

#[tokio::main]
async fn main() {
    init_logging();
    _ = server::launch_server().await;
    info!("Bootstrapping");
}

fn init_logging() {
    env_logger::init();
    println!("----------------------------------------------------------------------------------------|");
    println!("| | | Log level: {}", log::max_level());
    info!("Logging rabotaet");
}

