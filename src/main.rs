extern crate rustbox;
extern crate rand;

use rustbox::{ Color, RustBox };

use rustbox::keyboard::Key;
use control::Control;
use control_manager::ControlManager;
use std::default::Default;

mod control;
mod control_manager;
mod dice_roll;
mod dice;

static TITLE_STRING: &'static str = "Rust Roller - Tabletop rpg dice roller implemented in rust";

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e)
    };

    let mut control_manager = ControlManager::initialize(&rustbox,);
    control_manager.reposition();

    draw_screen(&rustbox);
    control_manager.redraw();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => break,
                    _ => {
                        handle_key(key, &mut control_manager);
                        control_manager.redraw();
                    }
                };
            },
            Ok(rustbox::Event::ResizeEvent(_, _)) => {
                draw_screen(&rustbox);
                control_manager.reposition();
                control_manager.redraw();
            },
            Err(e) => panic!("{:?}", e),
            _ => { }
        }
    }
}


fn handle_key(key: Key, control_manager: &mut ControlManager) {
    control_manager.handle_key(key);
}

fn draw_screen(rustbox: &RustBox) {

    rustbox.clear();

    rustbox.print(0, 0, rustbox::RB_UNDERLINE, Color::White, Color::Black, 
                  &format!("  {}{}", 
                          TITLE_STRING, 
                          std::iter::repeat(' ').take(256).collect::<String>()));

    // Draw bottom horizontal border
    rustbox.print(0, rustbox.height() - 2, rustbox::RB_NORMAL, Color::White, Color::Black,
                  &std::iter::repeat('=').take(rustbox.width()).collect::<String>());

    // Draw right history/saved vertical borders
    for i in 1usize..rustbox.height() - 2 {
        rustbox.print_char(rustbox.width() - 31, i, rustbox::RB_NORMAL, 
                           Color::White, Color::Black, '|');
        rustbox.print_char(rustbox.width() - 62, i, rustbox::RB_NORMAL, 
                           Color::White, Color::Black, '|');
    }

    rustbox.present();
}

