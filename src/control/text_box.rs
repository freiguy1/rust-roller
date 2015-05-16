#![allow(dead_code)]
#![allow(unused_variables)]

use rustbox::keyboard::Key;
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
            self.content.parse().unwrap()
        }
    }

    pub fn set_isize(&mut self, number: isize) {
        if number == 0 {
            self.content.clear();
            self.cursor_position = 0;
        } else {
            self.content = format!("{}", number);
            self.cursor_position = self.content.len()
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
        &self.content
    }

    fn drawable_content(&self) -> String {
        let num_spaces = self.content_max - self.content.len();
        format!("{}{}", self.content, 
                &::std::iter::repeat(' ')
                     .take(num_spaces)
                     .collect::<String>())

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
                      &::std::iter::repeat(' ')
                          .take(self.label.len() + 1 + self.content_max)
                          .collect::<String>());

        // draw label
        rustbox.print(self.x, 
                      self.y, 
                      ::rustbox::RB_NORMAL, 
                      Color::White, 
                      Color::Default, 
                      &self.label);

        // draw box
        let box_x = self.x + self.label.len() + 1;
        rustbox.print(
            box_x, 
                      self.y, 
                      ::rustbox::RB_UNDERLINE, 
                      Color::White,
                      Color::Default, 
                      &self.drawable_content());

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
                                           &(&self.content)[..self.cursor_position - 1],
                                           &(&self.content)[self.cursor_position..]);
                    self.cursor_position -= 1;
                }
            },
            Key::Char(c) => {
                if self.content.len() != self.content_max && (
                    c.is_digit(10) || (
                        !self.only_positive && (
                            c == '-' && self.cursor_position == 0 && (
                                self.content.len() == 0 || 
                                self.content.chars().next().unwrap() != '-'
                            )
                        )
                    )
                ){
                    self.content = format!("{}{}{}",
                                           &(&self.content)[..self.cursor_position],
                                           c,
                                           &(&self.content)[self.cursor_position..]);
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
