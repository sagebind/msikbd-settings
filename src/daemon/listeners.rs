//! Background services that listen for various types of events for triggering a config reload.
use config;
use simple_signal::{self, Signal};
use std::io;
use std::thread;
use std::sync::mpsc::*;


/// Start all listeners and return a receiver that can be waited on.
pub fn start() -> Receiver<()> {
    let (sender, receiver) = channel();

    let signal_sender = sender.clone();
    simple_signal::set_handler(&[Signal::Hup], move |_| {
        info!("received SIGHUP");
        signal_sender.send(()).ok();
    });

    let inotify_sender = sender.clone();
    thread::spawn(move || {
        if let Err(e) = inotify_listen(inotify_sender) {
            panic!("inotify error: {}", e);
        }
    });

    thread::spawn(move || {
        dbus_listen(sender);
    });

    receiver
}

/// Listens for changes on the config files using inotify.
pub fn inotify_listen(channel: Sender<()>) -> io::Result<()> {
    use inotify::*;

    let flags = WatchMask::CREATE | WatchMask::DELETE | WatchMask::MODIFY | WatchMask::ONLYDIR;
    let mut inotify = Inotify::init()?;
    inotify.add_watch(config::PATH, flags)?;

    let mut buffer = [0; 1024];

    loop {
        let events = inotify.read_events_blocking(&mut buffer)?;

        if events.count() > 0 {
            info!("detected config change");
            if channel.send(()).is_err() {
                break;
            }
        }
    }

    Ok(())
}

/// Listens for power-related signals over D-Bus.
pub fn dbus_listen(channel: Sender<()>) {
    use dbus::*;

    let bus = match Connection::get_private(BusType::System) {
        Ok(bus) => bus,
        Err(e) => {
            error!("error connecting to dbus: {}", e);
            return;
        }
    };

    'main: loop {
        for message in bus.iter(1000) {
            if let ConnectionItem::Signal(message) = message {
                let interface = message.interface().map(|i| i.to_string());
                let member = message.member().map(|m| m.to_string());

                info!("received signal: {:?}, {:?}", interface, member);

                if interface.as_ref().map(String::as_str) == Some("org.freedesktop.login1.Manager") &&
                    member.as_ref().map(String::as_str) == Some("PrepareForSleep") {
                    info!("detected sleep or wake up");

                    if channel.send(()).is_err() {
                        break 'main;
                    }
                }
            }
        }
    }
}
