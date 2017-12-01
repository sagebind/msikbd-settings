use config::Config;
use hidapi::HidDevice;


pub const VENDOR_ID: u16 = 0x1770;
pub const PRODUCT_ID: u16 = 0xff00;


pub fn set_config(device: &HidDevice, config: &Config) {
    if let Ok(mode) = config.get("mode") {
        set_mode(device, &mode);
    }
    if let Ok(color) = config.get("color-left") {
        set_color(device, "left", &color);
    }
    if let Ok(color) = config.get("color-middle") {
        set_color(device, "middle", &color);
    }
    if let Ok(color) = config.get("color-right") {
        set_color(device, "right", &color);
    }
}

pub fn set_brightness(device: &HidDevice) {}

pub fn set_color(device: &HidDevice, region: &str, color: &str) {
    let color = match parse_color(color.trim()) {
        Some(c) => c,
        None => {
            warn!("invalid color: {}", color);
            return;
        }
    };

    let region = match region {
        "left" => 1,
        "middle" => 2,
        "right" => 3,
        _ => {
            warn!("unknown color region: {}", region);
            return;
        }
    };

    send_feature_report(device, &[
        1,
        2,
        64, // rgb
        region,
        color.0,
        color.1,
        color.2,
        236, // EOR (end of request)
    ]);
}

pub fn set_mode(device: &HidDevice, mode: &str) {
    let mode = match mode.trim() {
        "normal" => 1,
        "gaming" => 2,
        "breathe" => 3,
        "demo" => 4,
        "wave" => 5,
        _ => {
            warn!("unknown mode: {}", mode);
            return;
        }
    };

    send_feature_report(device, &[
        1,
        2,
        65,
        mode,
        0,
        0,
        0,
        236, // EOR (end of request)
    ]);
}

fn send_feature_report(device: &HidDevice, buffer: &[u8]) {
    trace!("setting device feature: {:?}", buffer);

    if let Err(e) = device.send_feature_report(buffer) {
        error!("error setting device feature: {}", e);
    }
}

fn parse_color(color: &str) -> Option<(u8, u8, u8)> {
    if color.len() != 6 {
        return None;
    }

    let red = u8::from_str_radix(&color[0..2], 16).ok()?;
    let green = u8::from_str_radix(&color[2..4], 16).ok()?;
    let blue = u8::from_str_radix(&color[4..6], 16).ok()?;

    Some((red, green, blue))
}
