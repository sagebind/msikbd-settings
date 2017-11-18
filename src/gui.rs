use api;
use gtk;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;


pub fn main() {
    if gtk::init().is_err() {
        error!("Failed to initialize GTK.");
        return;
    }

    let client = match api::Client::connect() {
        Ok(v) => Rc::new(RefCell::new(v)),
        Err(e) => {
            error!("could not connect to msikbd service");
            return;
        },
    };

    let builder = gtk::Builder::new_from_string(include_str!("gui.glade"));

    let window: gtk::Window = builder.get_object("window").unwrap();
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    {
        let client = client.clone();
        let left_color: gtk::ColorButton = builder.get_object("left_color").unwrap();
        left_color.connect_color_set(move |color| {
            let rgba = color.get_rgba();
            let color = api::Color(color_f64_to_u8(rgba.red), color_f64_to_u8(rgba.green), color_f64_to_u8(rgba.blue));

            client.borrow_mut().set_color(api::Region::Left, color);
        });
    }

    {
        let client = client.clone();
        let center_color: gtk::ColorButton = builder.get_object("center_color").unwrap();
        center_color.connect_color_set(move |color| {
            let rgba = color.get_rgba();
            let color = api::Color(color_f64_to_u8(rgba.red), color_f64_to_u8(rgba.green), color_f64_to_u8(rgba.blue));

            client.borrow_mut().set_color(api::Region::Middle, color);
        });
    }

    {
        let client = client.clone();
        let right_color: gtk::ColorButton = builder.get_object("right_color").unwrap();
        right_color.connect_color_set(move |color| {
            let rgba = color.get_rgba();
            let color = api::Color(color_f64_to_u8(rgba.red), color_f64_to_u8(rgba.green), color_f64_to_u8(rgba.blue));

            client.borrow_mut().set_color(api::Region::Right, color);
        });
    }

    {
        let mode_select: gtk::ComboBoxText = builder.get_object("mode_select").unwrap();
        mode_select.connect_changed(move |select| {
            if let Some(mode) = select.get_active_id().and_then(|s| api::Mode::from_str(&s)) {
                client.borrow_mut().set_mode(mode);
            }
        });
    }

    gtk::main();
}

fn color_f64_to_u8(color: f64) -> u8 {
    (color * 255f64) as u8
}
