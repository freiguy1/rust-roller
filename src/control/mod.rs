use ::keyboard::Key;
use rustbox::RustBox;

pub use self::text_box::TextBox;
pub use self::list_select::ListSelect;

mod text_box;
mod list_select;

pub trait Control {
    fn redraw(&self, rustbox: &RustBox);
    fn clear_data(&mut self);
    fn set_selected(&mut self, selected: bool);
    fn handle_key<T: ::controls::ControlCallback>(&mut self, key: Key, callback: &T);
    fn set_size(&mut self, x: usize, y: usize);
    fn set_location(&mut self, x: usize, y: usize);
}
/*
pub enum ControlTypes {
    TextBox(TextBox),
    ListSelect(ListSelect)
}

impl ControlTypes {
    fn control(&self) -> &Control {
       match *self {
            ControlTypes::TextBox(ref tb) => tb,
            ControlTypes::ListSelect(ref dd) => dd
        }
    }

    fn control_mut(&mut self) -> &mut Control {
       match *self {
            ControlTypes::TextBox(ref mut tb) => tb,
            ControlTypes::ListSelect(ref mut dd) => dd
        }
    }
}

impl Control for ControlTypes {

    fn redraw(&self, rustbox: &RustBox) {
        self.control().redraw(rustbox)
    }

    fn clear_data(&mut self) {
        self.control_mut().clear_data();
    }

    fn handle_key(&mut self, key: Key) {
        self.control_mut().handle_key(key);
    }
}

*/
