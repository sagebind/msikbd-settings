use inotify::*;
use std::fs::*;
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::*;


pub struct Config {
    dir: PathBuf,
    inotify: Option<Inotify>,
}

impl Config {
    pub fn open() -> Config {
        Self::open_at("/etc/msikbd")
    }

    pub fn open_at<P: Into<PathBuf>>(dir: P) -> Config {
        let dir = dir.into();

        if !dir.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(&dir)
                .unwrap();
        }

        Config {
            dir: dir,
            inotify: None,
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.dir.join(key).exists()
    }

    pub fn get(&self, key: &str) -> io::Result<String> {
        let path = self.dir.join(key);
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn set(&self, key: &str, value: &str) -> io::Result<()> {
        let path = self.dir.join(key);
        let mut file = File::create(path)?;

        // Set file permissions if we can. Ignore errors.
        file.set_permissions(PermissionsExt::from_mode(0o666)).ok();

        file.write_all(value.as_bytes())?;

        Ok(())
    }

    pub fn populate_defaults(&self) -> io::Result<()> {
        if !self.contains("mode") {
            self.set("mode", "normal")?;
        }

        for key in &["color-left", "color-middle", "color-right"] {
            if !self.contains(key) {
                self.set(key, "ff0000")?;
            }
        }

        Ok(())
    }

    pub fn wait_for_change(&mut self) -> io::Result<()> {
        if self.inotify.is_none() {
            let flags = WatchMask::CREATE | WatchMask::DELETE | WatchMask::MODIFY | WatchMask::ONLYDIR;
            let mut inotify = Inotify::init()?;
            inotify.add_watch(&self.dir, flags)?;
            self.inotify = Some(inotify);
        }

        let mut buffer = [0; 1024];

        loop {
            let events = self.inotify.as_mut().unwrap().read_events_blocking(&mut buffer)?;
            if events.count() > 0 {
                break;
            }
        }

        Ok(())
    }
}
