#![allow(unstable)]

extern crate rustbox;

use std::char;
use std::io::stdio;
use std::error::Error;

use rustbox::{Color, RustBox, InitOption};

use keyboard::Key;
use control::Control;
use controls::Controls;

mod keyboard;
mod control;
mod controls;

static TITLE_STRING: &'static str = "Rust Roller - Tabletop rpg dice roller implemented in rust";

fn main() {
    let options = [
        if stdio::stderr_raw().isatty() { Some(InitOption::BufferStderr) } else { None },
    ];
    let rustbox = RustBox::init(&options).unwrap();

    let mut controls = Controls::initialize(rustbox.height());
    controls.reposition(rustbox.height(), rustbox.width());

    draw_screen(&rustbox);
    controls.redraw(&rustbox);

    loop {
        match rustbox.poll_event() {
            Ok(rustbox::Event::KeyEvent(_, key, ch)) => {
                let k = match key {
                    0 => {
                        let mut exit = false;
                        let temp = char::from_u32(ch).map(|c| {
                            if c == 'q' { exit = true; }
                            Key::Char(c)
                        });
                        if exit { break; }
                        temp
                    },
                    a => Key::from_special_code(a),
                };
                handle_key(k, &mut controls);
                controls.redraw(&rustbox);
            },
            Ok(rustbox::Event::ResizeEvent(_, _)) => {
                draw_screen(&rustbox);
                controls.reposition(rustbox.height(), rustbox.width());
                controls.redraw(&rustbox);
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}


fn handle_key(key: Option<Key>, controls: &mut Controls) {
    match key {
        Some(some_key) => {
            match some_key {
                Key::Backspace => controls.handle_key(some_key),
                Key::Char(_) => controls.handle_key(some_key),
                Key::Left => controls.handle_key(some_key),
                Key::Right => controls.handle_key(some_key),
                Key::Tab => controls.select_next(),
                _ => ()
            }
        },
        None => ()
    }
}

fn draw_screen(rustbox: &RustBox) {

    rustbox.clear();

    rustbox.print(0, 0, rustbox::RB_UNDERLINE, Color::White, Color::Black, 
                  format!("  {}{}", 
                          TITLE_STRING, 
                          std::iter::repeat(' ').take(256).collect::<String>()).as_slice());

    // Draw bottom horizontal border
    rustbox.print(0, rustbox.height() - 2, rustbox::RB_NORMAL, Color::White, Color::Black,
                  std::iter::repeat('=').take(rustbox.width()).collect::<String>().as_slice());

    // Draw right history/saved vertical borders
    for i in range(1us, rustbox.height() - 2) {
        rustbox.print_char(rustbox.width() - 20, i, rustbox::RB_NORMAL, 
                           Color::White, Color::Black, '|');
        rustbox.print_char(rustbox.width() - 40, i, rustbox::RB_NORMAL, 
                           Color::White, Color::Black, '|');
        if i == 1 {
            rustbox.print(rustbox.width() - 19, 1, rustbox::RB_BOLD,
                          Color::White, Color::Black, "      History");
            rustbox.print(rustbox.width() - 39, 1, rustbox::RB_BOLD,
                          Color::White, Color::Black, "       Saved");
        }
    }

    rustbox.present();
}

