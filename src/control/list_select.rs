#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use ::keyboard::Key;
use rustbox::{ Color, RustBox };
use ::control::Control;


pub struct ListSelect {
    items: Vec<String>,
    selected_index: Option<usize>
}

impl Control for ListSelect {
    fn redraw(&self, rustbox: &RustBox) {}
    fn clear_data(&mut self) {}
    fn handle_key<T: ::controls::ControlCallback>(&mut self, key: Key, callback: &T) {}
    fn set_selected(&mut self, selected: bool){}
    fn set_size(&mut self, x: usize, y: usize){}
    fn set_location(&mut self, x: usize, y: usize){}
}
