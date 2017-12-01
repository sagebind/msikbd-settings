use gdk::RGBA;


pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn from_hex<S: AsRef<str>>(color: S) -> Option<Color> {
        let color = color.as_ref();
        if color.len() != 6 {
            return None;
        }

        let red = u8::from_str_radix(&color[0..2], 16).ok()?;
        let green = u8::from_str_radix(&color[2..4], 16).ok()?;
        let blue = u8::from_str_radix(&color[4..6], 16).ok()?;

        Some(Color(red, green, blue))
    }

    pub fn to_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

impl From<RGBA> for Color {
    fn from(rgba: RGBA) -> Color {
        Color(
            (rgba.red * 255f64) as u8,
            (rgba.green * 255f64) as u8,
            (rgba.blue * 255f64) as u8,
        )
    }
}

impl Into<RGBA> for Color {
    fn into(self) -> RGBA {
        RGBA {
            red: (self.0 as f64) / 255f64,
            green: (self.1 as f64) / 255f64,
            blue: (self.2 as f64) / 255f64,
            alpha: 1f64,
        }
    }
}
