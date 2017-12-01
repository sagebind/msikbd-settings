use config::Config;
use hidapi::*;
use keyboard;


pub fn main() {
    // Attempt to open the keyboard device.
    let hid = HidApi::new().unwrap();
    let device = match hid.open(keyboard::VENDOR_ID, keyboard::PRODUCT_ID) {
        Ok(d) => d,
        Err(_) => {
            error!("device not found");
            return;
        },
    };

    let mut config = Config::open();

    loop {
        config.populate_defaults().unwrap();

        info!("reloading configuration");
        keyboard::set_config(&device, &config);

        config.wait_for_change().unwrap();
    }
}
