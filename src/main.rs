extern crate clap;
extern crate env_logger;
extern crate gtk;
extern crate hidapi;
extern crate libc;
#[macro_use]
extern crate log;

mod api;
mod daemon;
mod gui;
mod pipe;

use clap::*;
use std::env;


fn init_logging() {
    let mut builder = env_logger::LogBuilder::new();
    builder.filter(None, log::LogLevelFilter::Info);

    if let Ok(v) = env::var("LOGLEVEL") {
        builder.parse(&v);
    }

    builder.init().expect("error initializing logging");
}

fn main() {
    init_logging();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("daemonize")
            .short("d")
            .long("daemon")
            .help("run as a device daemon"))
        .arg(Arg::with_name("gui")
            .long("gui")
            .help("Launch the GUI"))
        .get_matches();

    if matches.is_present("gui") {
        gui::main();
    }

    if matches.is_present("daemonize") {
        daemon::main();
    }
}
