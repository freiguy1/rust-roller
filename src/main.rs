#![allow(unstable)]

extern crate rustbox;

use std::char;
use std::io::stdio;

use rustbox::{Color, RustBox, InitOption};

use keyboard::Key;
use control::Control;
use controls::Controls;

mod keyboard;
mod control;
mod controls;
mod dice_roll;
mod dice;

static TITLE_STRING: &'static str = "Rust Roller - Tabletop rpg dice roller implemented in rust";

fn main() {
    let options = [
        if stdio::stderr_raw().isatty() { Some(InitOption::BufferStderr) } else { None },
    ];
    let rustbox = RustBox::init(&options).ok().unwrap();

    let mut controls = Controls::initialize(&rustbox,);
    controls.reposition();

    draw_screen(&rustbox);
    controls.redraw();

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
                controls.redraw();
            },
            Ok(rustbox::Event::ResizeEvent(_, _)) => {
                draw_screen(&rustbox);
                controls.reposition();
                controls.redraw();
            },
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }
}


fn handle_key(key: Option<Key>, controls: &mut Controls) {
    match key {
        Some(some_key) => controls.handle_key(some_key),
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
        rustbox.print_char(rustbox.width() - 31, i, rustbox::RB_NORMAL, 
                           Color::White, Color::Black, '|');
        rustbox.print_char(rustbox.width() - 62, i, rustbox::RB_NORMAL, 
                           Color::White, Color::Black, '|');
    }

    rustbox.present();
}

