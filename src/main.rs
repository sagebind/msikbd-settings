extern crate clap;
extern crate dbus;
extern crate env_logger;
extern crate gdk;
extern crate gtk;
extern crate hidapi;
extern crate inotify;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate simple_signal;

mod color;
mod config;
mod daemon;
mod client;

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
        .arg(Arg::with_name("get")
            .long("get")
            .value_name("KEY")
            .help("Get the value of a setting"))
        .arg(Arg::with_name("set")
            .long("set")
            .value_name("KEY")
            .help("Set a setting value"))
        .arg(Arg::with_name("daemon")
            .short("d")
            .long("daemon")
            .help("run as a device daemon"))
        .arg(Arg::with_name("gui")
            .long("gui")
            .help("Launch the GUI"))
        .get_matches();

    if matches.is_present("daemon") {
        daemon::main();
        return;
    }

    if let Some(key) = matches.value_of("get") {
        if let Ok(value) = config::get(key) {
            println!("{}", value);
        }
        return;
    }

    if let Some(set) = matches.value_of("set") {
        if let Some(index) = set.find('=') {
            let (key, value) = set.split_at(index);
            config::set(key, &value[1..]);
        }
        return;
    }

    client::gui::main();
}
