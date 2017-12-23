use config;

mod keyboard;
mod listeners;


pub fn main() {
    // Spawn change listeners.
    let receiver = listeners::start();

    loop {
        config::populate_defaults().unwrap();

        info!("reloading configuration");
        keyboard::apply_config();

        if receiver.recv().is_err() {
            info!("listeners hung up, exiting");
            break;
        }
    }
}
