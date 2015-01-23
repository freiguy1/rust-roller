#![allow(dead_code)]
#![allow(unused_variables)]

use ::keyboard::Key;
use rustbox::{ Color, RustBox };
use ::control::Control;

pub struct TextBox {
    content: String,
    label: String,
    content_max: usize,
    x: usize,
    y: usize,
    pub selected: bool,
    cursor_position: usize,
    only_positive: bool
}

impl TextBox {
    pub fn get_isize(&self) -> isize {
        if self.content.len() == 0 {
            0
        } else {
            ::std::num::from_str_radix::<isize>(self.content.as_slice(), 10).unwrap()
        }
    }

    pub fn new(label: &str, content_max: usize, only_pos: bool) -> Self {
        TextBox {
            content: "".to_string(),
            label: label.to_string(),
            content_max: content_max,
            x: 0,
            y: 0,
            selected: false,
            cursor_position: 0,
            only_positive: only_pos
        }
    }

    pub fn get_content(&self) -> &str {
        self.content.as_slice()
    }

    fn drawable_content(&self) -> String {
        let num_spaces = self.content_max - self.content.len();
        format!("{}{}", self.content, 
                ::std::iter::repeat(' ')
                     .take(num_spaces)
                     .collect::<String>()
                     .as_slice())

    }
}

impl Control for TextBox {
    fn redraw(&self, rustbox: &RustBox) {
        // clear
        rustbox.print(self.x,
                      self.y,
                      ::rustbox::RB_NORMAL,
                      Color::White,
                      Color::Black,
                      ::std::iter::repeat(' ')
                          .take(self.label.len() + 1 + self.content_max)
                          .collect::<String>()
                          .as_slice());

        // draw label
        rustbox.print(self.x, 
                      self.y, 
                      ::rustbox::RB_NORMAL, 
                      Color::White, 
                      Color::Black, 
                      self.label.as_slice());

        // draw box
        let box_x = self.x + self.label.len() + 1;
        rustbox.print(
            box_x, 
                      self.y, 
                      ::rustbox::RB_NORMAL, 
                      Color::Black, 
                      Color::White, 
                      self.drawable_content().as_slice());

        if self.selected {
            rustbox.set_cursor((self.x + self.label.len() + 1 + self.cursor_position) as isize, self.y as isize)
        }
    }

    fn clear_data(&mut self) {
        self.content = String::new();
        self.cursor_position = 0;
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Backspace => {
                if self.cursor_position != 0 {
                    self.content = format!("{}{}",
                                           self.content.as_slice().slice_to(self.cursor_position - 1),
                                           self.content.as_slice().slice_from(self.cursor_position));
                    self.cursor_position -= 1;
                }
            },
            Key::Char(c) => {
                if self.content.len() != self.content_max && (
                    c.is_digit(10) || (
                        !self.only_positive && (
                            c == '-' && self.cursor_position == 0 && (
                                self.content.len() == 0 || 
                                self.content.as_slice().char_at(0) != '-'
                            )
                        )
                    )
                ){
                    self.content = format!("{}{}{}",
                                           self.content.as_slice().slice_to(self.cursor_position),
                                           c,
                                           self.content.as_slice().slice_from(self.cursor_position));
                    self.cursor_position += 1;
                }
            },
            Key::Right => {
                if self.cursor_position != self.content_max &&
                    self.cursor_position < self.content.len() {
                    self.cursor_position += 1;
                }
            },
            Key::Left => {
                if self.cursor_position != 0 {
                    self.cursor_position -= 1;
                }
            }
            _ => ()
        }
    }

    fn set_selected(&mut self, selected: bool){
        self.selected = selected;
    }

    fn set_size(&mut self, _: usize, _: usize){}

    fn set_location(&mut self, x: usize, y: usize){
        self.x = x;
        self.y = y;
    }
}
