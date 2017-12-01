use color::Color;
use config::Config;
use gtk;
use gtk::prelude::*;
use std::rc::Rc;


pub fn main() {
    if gtk::init().is_err() {
        error!("Failed to initialize GTK.");
        return;
    }

    let config = Rc::new(Config::open());
    let builder = gtk::Builder::new_from_string(include_str!("gui.glade"));

    let window: gtk::Window = builder.get_object("window").unwrap();
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    {
        let config = config.clone();
        let left_color: gtk::ColorButton = builder.get_object("left_color").unwrap();

        if let Some(color) = config.get("color-left").ok().and_then(Color::from_hex) {
            left_color.set_rgba(&color.into());
        }

        left_color.connect_color_set(move |color| {
            let rgba = color.get_rgba();
            let color = hex_color(rgba.red, rgba.green, rgba.blue);
            config.set("color-left", &color);
        });
    }

    {
        let config = config.clone();
        let center_color: gtk::ColorButton = builder.get_object("center_color").unwrap();

        if let Some(color) = config.get("color-middle").ok().and_then(Color::from_hex) {
            center_color.set_rgba(&color.into());
        }

        center_color.connect_color_set(move |color| {
            let rgba = color.get_rgba();
            let color = hex_color(rgba.red, rgba.green, rgba.blue);
            config.set("color-middle", &color);
        });
    }

    {
        let config = config.clone();
        let right_color: gtk::ColorButton = builder.get_object("right_color").unwrap();

        if let Some(color) = config.get("color-right").ok().and_then(Color::from_hex) {
            right_color.set_rgba(&color.into());
        }

        right_color.connect_color_set(move |color| {
            let rgba = color.get_rgba();
            let color = hex_color(rgba.red, rgba.green, rgba.blue);
            config.set("color-right", &color);
        });
    }

    {
        let mode_select: gtk::ComboBoxText = builder.get_object("mode_select").unwrap();
        mode_select.connect_changed(move |select| {
            if let Some(mode) = select.get_active_id() {
                config.set("mode", &mode);
            }
        });
    }

    gtk::main();
}

fn hex_color(red: f64, green: f64, blue: f64) -> String {
    format!(
        "{:02x}{:02x}{:02x}",
        color_f64_to_u8(red),
        color_f64_to_u8(green),
        color_f64_to_u8(blue),
    )
}

fn color_f64_to_u8(color: f64) -> u8 {
    (color * 255f64) as u8
}
