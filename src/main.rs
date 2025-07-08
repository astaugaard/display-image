use std::{
    fs::File,
    io::{self, Read},
};

use clap::Parser;
use gio::{
    glib::{Char, OptionArg, OptionFlags, Propagation},
    prelude::*,
    ApplicationFlags,
};
use gtk4::{
    gdk::{prelude::TextureExt, Display, Texture},
    prelude::{BoxExt, GtkWindowExt, WidgetExt},
    Align, CssProvider, EventControllerKey, GestureClick, Image,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    css: Option<String>,

    #[arg(short, long)]
    image: String,

    #[arg(short, long)]
    size: usize,
}

fn activate(application: &gtk4::Application, image: &[u8], size: usize) {
    let window = gtk4::ApplicationWindow::new(application);

    window.init_layer_shell();

    window.set_layer(Layer::Top);

    window.fullscreen();

    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    let click_gesture = GestureClick::builder().button(0).build();
    let key = EventControllerKey::new();

    let win2 = window.clone();

    key.connect_key_pressed(move |_, _, _, _| {
        win2.close();
        Propagation::Stop
    });

    window.add_controller(key);

    let win2 = window.clone();

    click_gesture.connect_pressed(move |_, _, _, _| {
        win2.close();
    });

    window.add_controller(click_gesture);

    let b = gtk4::Box::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .build();

    let texture = Texture::from_bytes(&image.into()).unwrap();

    let width = texture.width();

    let height = texture.height();

    let a = width.max(height);

    let width = (width * size as i32) / a;

    let height = (height * size as i32) / a;

    let image = Image::builder()
        .height_request(height)
        .width_request(width)
        .paintable(&texture)
        .build();

    b.append(&image);

    window.set_child(Some(&b));

    window.set_visible(true)
}

fn main() {
    let Args { css, image, size } = dbg!(Args::parse());

    let mut iimage = Vec::new();

    if &image == "-" {
        io::stdin()
            .lock()
            .read_to_end(&mut iimage)
            .expect("need valid image");
    } else {
        File::open(image)
            .ok()
            .unwrap()
            .read_to_end(&mut iimage)
            .expect("need valid image");
    }

    let mut flags = ApplicationFlags::empty();

    flags.set(ApplicationFlags::HANDLES_COMMAND_LINE, true);

    let application = gtk4::Application::builder().build();

    application.add_main_option(
        "image",
        Char::from(b'i'),
        OptionFlags::NONE,
        OptionArg::String,
        "image to display",
        Some("COMMAND"),
    );

    application.add_main_option(
        "css",
        Char::from(b'c'),
        OptionFlags::NONE,
        OptionArg::String,
        "style sheet to use",
        Some("STYLE"),
    );

    application.add_main_option(
        "size",
        Char::from(b's'),
        OptionFlags::NONE,
        OptionArg::String,
        "height",
        Some("STYLE"),
    );

    application.connect_command_line(|_app, _cli| 0); // idk why I have to do this tbh

    application.connect_startup(move |_| {
        let provider = CssProvider::new();

        match &css {
            Some(file) => {
                provider.load_from_path(file);
            }
            None => {
                provider.load_from_string(include_str!("style.css"));
            }
        };

        gtk4::style_context_add_provider_for_display(
            &Display::default().expect("could not connect to a display."),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        )
    });

    application.connect_activate(move |app| {
        activate(app, &iimage, size);
    });

    application.run();
}
