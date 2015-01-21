#![allow(unstable)]

extern crate rustbox;

use std::char;
use std::io::stdio;
use std::error::Error;

use rustbox::{Color, RustBox, InitOption};

use keyboard::Key;
use control::Control;

mod keyboard;
mod control;

static TITLE_STRING: &'static str = "Rust Roller - DnD dice roller implemented in rust";

fn main() {
    let options = [
        if stdio::stderr_raw().isatty() { Some(InitOption::BufferStderr) } else { None },
    ];
    let rustbox = RustBox::init(&options).unwrap();

    draw_screen(&rustbox);

    loop {
        match rustbox.poll_event() {
            Ok(rustbox::Event::KeyEvent(_, key, ch)) => {
                println!("{} {}", key, ch);
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
                handle_key(k);
            },
            Ok(rustbox::Event::ResizeEvent(_, _)) => {
                draw_screen(&rustbox);
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}

fn handle_key(key: Option<Key>) {
/*
Tab,
Enter,
Esc,
Backspace,
Right,
Left,
Down,
Up,
Delete,
Char(char),
Ctrl(char),
*/

}

fn draw_screen(rustbox: &RustBox) {
    //let width = rustbox.width();
    //let height = rustbox.height();

    rustbox.clear();

    rustbox.print(0, 0, rustbox::RB_UNDERLINE, Color::White, Color::Black, 
                  format!("  {}{}", 
                          TITLE_STRING, 
                          std::iter::repeat(' ').take(256).collect::<String>()).as_slice());

    draw_view(rustbox);

    rustbox.present();
}

fn draw_view(rustbox: &RustBox) {
    let mut top_y = rustbox.height() / 2 - 6;
    if top_y <= 0 {
        top_y = 2;
    }

    let mut tb_d4 = control::TextBox::new(" d4", 20, 1, top_y);
    tb_d4.set_selected(true);
    /*let mut tb_d6 = control::TextBox::new(" d6", 2, 1, top_y + 2);
    let mut tb_d8 = control::TextBox::new(" d8", 2, 1, top_y + 4);
    let mut tb_d10 = control::TextBox::new("d10", 2, 1, top_y + 6);
    let mut tb_d20 = control::TextBox::new("d20", 2, 1, top_y + 8);
    let mut tb_mod = control::TextBox::new("mod", 2, 1, top_y + 10);
*/
    tb_d4.redraw(rustbox);
    tb_d4.handle_key(Key::Backspace);
    tb_d4.redraw(rustbox);

 /*   tb_d6.redraw(rustbox);
    tb_d8.redraw(rustbox);
    tb_d10.redraw(rustbox);
    tb_d20.redraw(rustbox);
    tb_mod.redraw(rustbox);
    */


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

}
