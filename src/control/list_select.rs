#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use control::Control;
use dice::Dice;
use rustbox::keyboard::Key;
use rustbox::{Color, RustBox};
use std::collections::VecDeque;

pub struct ListSelect {
    items: VecDeque<Dice>,
    selected_index: usize,
    selected: bool,
    size_x: usize,
    size_y: usize,
    loc_x: usize,
    loc_y: usize,
    name: String,
}

impl ListSelect {
    pub fn new(name: &str) -> ListSelect {
        ListSelect {
            items: VecDeque::new(),
            selected_index: 0,
            selected: false,
            size_x: 0,
            size_y: 0,
            loc_x: 0,
            loc_y: 0,
            name: String::from(name),
        }
    }

    pub fn add_item(&mut self, item: Dice) {
        match self.items.iter().position(|dice| *dice == item) {
            Some(i) if i > 0 => {
                self.items.remove(i);
                self.items.push_front(item);
            }
            Some(_) => {}
            None => {
                self.items.push_front(item);
            }
        }
    }

    pub fn selected_item(&self) -> Option<Dice> {
        self.items.get(self.selected_index).map(|&dice| dice)
    }

    pub fn has_items(&self) -> bool {
        self.items.len() != 0
    }

    pub fn correct_length_string(dice: &Dice, length: usize) -> String {
        let mut result = format!("{:?}", dice);
        let extra_spaces: isize = length as isize - result.len() as isize;
        if extra_spaces > 0 {
            result.push_str(
                &::std::iter::repeat(' ')
                    .take(extra_spaces as usize)
                    .collect::<String>(),
            );
            result
        } else {
            String::from(&(&result)[..length])
        }
    }
}

impl Control for ListSelect {
    fn redraw(&self, rustbox: &RustBox) {
        // Draw Title
        let mut title_start_x = self.loc_x + (self.size_x / 2 - (self.name.len() / 2));
        if title_start_x < self.loc_x {
            title_start_x = 0;
        }
        let draw_title: &str = match self.name.len() {
            len if len > self.size_x => &(&self.name)[..self.size_x],
            _ => &self.name,
        };
        rustbox.print(
            title_start_x,
            self.loc_y,
            ::rustbox::RB_BOLD,
            Color::White,
            Color::Black,
            draw_title,
        );

        for i in 1..self.size_y {
            rustbox.print(
                self.loc_x,
                self.loc_y + i,
                ::rustbox::RB_NORMAL,
                Color::White,
                Color::Black,
                &::std::iter::repeat(' ')
                    .take(self.size_x)
                    .collect::<String>(),
            );
        }

        // Draw items highlighting selected
        for (i, dice) in self.items.iter().take(self.size_y - 1).enumerate() {
            if self.selected && i == self.selected_index {
                rustbox.print(
                    self.loc_x,
                    self.loc_y + i + 1,
                    ::rustbox::RB_NORMAL,
                    Color::Black,
                    Color::White,
                    &ListSelect::correct_length_string(dice, self.size_x),
                );
                rustbox.set_cursor(-1, -1);
            } else {
                rustbox.print(
                    self.loc_x,
                    self.loc_y + i + 1,
                    ::rustbox::RB_NORMAL,
                    Color::White,
                    Color::Black,
                    &ListSelect::correct_length_string(dice, self.size_x),
                );
            }
        }
    }

    fn clear_data(&mut self) {
        self.selected_index = 0;
        self.items.clear();
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Down => {
                if self.selected_index != self.items.len() - 1 {
                    self.selected_index += 1;
                }
            }
            Key::Up => {
                if self.selected_index != 0 {
                    self.selected_index -= 1;
                }
            }
            _ => (),
        }
    }

    fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
    fn set_size(&mut self, x: usize, y: usize) {
        self.size_x = x;
        self.size_y = y;
    }
    fn set_location(&mut self, x: usize, y: usize) {
        self.loc_x = x;
        self.loc_y = y;
    }
}
