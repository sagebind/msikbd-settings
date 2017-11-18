use std::fs;
use std::io::{self, Write};
use std::path::*;


pub const DEFAULT_PIPE: &str = "/tmp/msikbd";


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Normal,
    Gaming,
    Breathe,
    Demo,
    Wave,
}

impl Mode {
    pub fn from_str(s: &str) -> Option<Mode> {
        match s {
            "normal" => Some(Mode::Normal),
            "gaming" => Some(Mode::Gaming),
            "breathe" => Some(Mode::Breathe),
            "demo" => Some(Mode::Demo),
            "wave" => Some(Mode::Wave),
            _ => None,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            &Mode::Normal => "normal",
            &Mode::Gaming => "gaming",
            &Mode::Breathe => "breathe",
            &Mode::Demo => "demo",
            &Mode::Wave => "wave",
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Region {
    Left,
    Middle,
    Right,
}

impl Region {
    pub fn from_str(s: &str) -> Option<Region> {
        match s {
            "left" => Some(Region::Left),
            "middle" => Some(Region::Middle),
            "right" => Some(Region::Right),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            &Region::Left => "left",
            &Region::Middle => "middle",
            &Region::Right => "right",
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);


pub struct Client {
    file: fs::File,
}

impl Client {
    pub fn connect() -> io::Result<Client> {
        Client::open(DEFAULT_PIPE)
    }

    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Client> {
        let file = fs::OpenOptions::new()
            .append(true)
            .open(&path)?;

        debug!("client connected: {}", path.as_ref().to_string_lossy());

        Ok(Client {
            file: file,
        })
    }

    pub fn set_color(&mut self, region: Region, color: Color) -> io::Result<()> {
        writeln!(self.file, "color {} {} {} {}", region.as_str(), color.0, color.1, color.2)
    }

    pub fn set_mode(&mut self, mode: Mode) -> io::Result<()> {
        writeln!(self.file, "mode {}", mode.as_str())
    }
}
