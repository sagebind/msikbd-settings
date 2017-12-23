use std::fs::*;
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::*;


pub const PATH: &str = "/etc/msikbd";


pub fn path() -> &'static Path {
    PATH.as_ref()
}

pub fn contains(key: &str) -> bool {
    path().join(key).exists()
}

pub fn get(key: &str) -> io::Result<String> {
    let path = path().join(key);
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    let contents = contents.trim().to_owned();
    Ok(contents)
}

pub fn set(key: &str, value: &str) -> io::Result<()> {
    let path = path().join(key);
    let mut file = File::create(path)?;

    // Set file permissions if we can. Ignore errors.
    if let Err(e) = file.set_permissions(PermissionsExt::from_mode(0o666)) {
        warn!("could not set file permissions: {}", e);
    }

    file.write_all(value.as_bytes())?;

    Ok(())
}

pub fn populate_defaults() -> io::Result<()> {
    let dir = path();

    if !dir.exists() {
        DirBuilder::new()
            .recursive(true)
            .create(&dir)
            .unwrap();
    }

    if !contains("mode") {
        set("mode", "normal")?;
    }

    for key in &["color-left", "color-middle", "color-right"] {
        if !contains(key) {
            set(key, "ff0000")?;
        }
    }

    Ok(())
}
