#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::Write;
use chrono::Local;
use colored::{ColoredString, Colorize};
use log::Level;

mod server;
pub mod service;

#[tokio::main]
async fn main() {
    init_logging();
    _ = server::launch_server().await;
    info!("Bootstrapping");
}

fn init_logging() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}{}:\t{}", Local::now().format("%d/%m/%Y %H:%M "), colourful_loglevel(record.level()), record.args()))
        .init();

    println!(
        "----------------------------------------------------------------------------------------|"
    );
    println!("| | | Log level: {}", log::max_level());
    info!("Logging rabotaet");   
}

fn colourful_loglevel(level: Level) -> ColoredString {
    match level {
        Level::Error => level.to_string().red(),
        Level::Warn => level.to_string().yellow(),
        Level::Info => level.to_string().blue(),
        Level::Debug => level.to_string().cyan(),
        Level::Trace => level.to_string().magenta(),
    }
}
