use hidapi::*;
use api::*;
use pipe;
use std::io::*;
use std::fs;
use std::path::*;
use std::os::unix::fs::PermissionsExt;


const VENDOR_ID: u16 = 0x1770;
const PRODUCT_ID: u16 = 0xff00;


pub fn main() {
    // Attempt to open the keyboard device.
    let hid = HidApi::new().unwrap();
    let device = match hid.open(VENDOR_ID, PRODUCT_ID) {
        Ok(d) => d,
        Err(_) => {
            error!("device not found");
            return;
        },
    };

    // Set up a named pipe for communicating with the daemon.
    info!("using named pipe: {}", DEFAULT_PIPE);
    let path = PathBuf::from(DEFAULT_PIPE);
    if path.exists() {
        trace!("removing existing file: {}", DEFAULT_PIPE);
        fs::remove_file(&path);
    }
    pipe::create_named_pipe(&path, PermissionsExt::from_mode(0o666)).unwrap();

    // Start reading commands from the pipe.
    let pipe = fs::OpenOptions::new()
        .read(true)
        .write(true) // enable writing so we never receive EOF
        .open(path)
        .unwrap();
    let mut reader = BufReader::new(pipe);
    let mut line = String::new();

    info!("service ready");
    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => continue,
            Err(e) => {
                eprintln!("error reading from pipe: {}", e);
                continue;
            }
            _ => (),
        }

        info!("received: {}", line.trim());

        let command: Vec<&str> = line.trim().split(' ').collect();
        handle_command(&device, &command);
    }
}

fn handle_command(device: &HidDevice, args: &[&str]) {
    match args.first() {
        Some(&"mode") => {
            if let Some(mode) = Mode::from_str(args[1]) {
                device_set_mode(device, mode);
            } else {
                warn!("unknown mode: {}", args[1])
            }
        }
        Some(&"color") => {
            let region = Region::from_str(args[1]).unwrap();
            let color = (args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap());
            device_set_color(device, region, color);
        }
        _ => {
            warn!("invalid command: {:?}", args.first());
        }
    }
}

fn device_set_brightness(device: &HidDevice) {}

fn device_set_color(device: &HidDevice, region: Region, color: (u8, u8, u8)) {
    send_feature_report(device, &[
        1,
        2,
        64, // rgb
        match region {
            Region::Left => 1,
            Region::Middle => 2,
            Region::Right => 3,
        },
        color.0,
        color.1,
        color.2,
        236, // EOR (end of request)
    ]);
}

fn device_set_mode(device: &HidDevice, mode: Mode) {
    send_feature_report(device, &[
        1,
        2,
        65,
        match mode {
            Mode::Normal => 1,
            Mode::Gaming => 2,
            Mode::Breathe => 3,
            Mode::Demo => 4,
            Mode::Wave => 5,
        },
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
