
use rustbox::{ RustBox };
use ::control::Control;

pub struct Controls {
    tb_d4: ::control::TextBox,
    tb_d6: ::control::TextBox,
    tb_d8: ::control::TextBox,
    tb_d10: ::control::TextBox,
    tb_d20: ::control::TextBox,
    tb_mod: ::control::TextBox,
    selected: usize
}

/*
impl<'a> Controls<'a> {
    fn new(tb_d4: ::control::TextBox) -> Self {
        let mut result = Controls { 
            tb_d4: tb_d4,
            /*tb_d6: tb_d4,
            tb_d8: tb_d4,
            tb_d10: tb_d4,
            tb_d20: tb_d4,
            tb_mod: tb_d4,*/
            control_order: Vec::new(),
            selected: 0
        };
        result.control_order.push(&result.tb_d4);
        result
    }
}
*/

impl Controls {

    pub fn initialize(height: usize) -> Self {
        let tb_d4 = ::control::TextBox::new(" d4", 20);
        let tb_d6 = ::control::TextBox::new(" d6", 2);
        let tb_d8 = ::control::TextBox::new(" d8", 2);
        let tb_d10 = ::control::TextBox::new("d10", 2);
        let tb_d20 = ::control::TextBox::new("d20", 2);
        let tb_mod = ::control::TextBox::new("mod", 2);

        let mut result = Controls {
            tb_d4: tb_d4,
            tb_d6: tb_d6,
            tb_d8: tb_d8,
            tb_d10: tb_d10,
            tb_d20: tb_d20,
            tb_mod: tb_mod,
            selected: 0
        };

        result.tb_d4.set_selected(true);
        result
    }

    pub fn reposition(&mut self, height: usize, width: usize) {
        let mut top_y = height / 2 - 6;
        if top_y <= 0 {
            top_y = 2;
        }

        self.tb_d4.set_location(1, top_y);
        self.tb_d6.set_location(1, top_y + 2);
        self.tb_d8.set_location(1, top_y + 4);
        self.tb_d10.set_location(1, top_y + 6);
        self.tb_d20.set_location(1, top_y + 8);
        self.tb_mod.set_location(1, top_y + 10);
    }

    pub fn redraw(&self, rustbox: &RustBox) {
        self.tb_d4.redraw(rustbox);
        self.tb_d6.redraw(rustbox);
        self.tb_d8.redraw(rustbox);
        self.tb_d10.redraw(rustbox);
        self.tb_d20.redraw(rustbox);
        self.tb_mod.redraw(rustbox);
        rustbox.present();
    }

    pub fn select_next(&mut self) {
        self.selected_control_mut().set_selected(false);
        self.selected += 1;
        if self.selected == 6 { self.selected = 0; }
        self.selected_control_mut().set_selected(true);
    }

    fn selected_control_mut(&mut self) -> &mut Control {
        match self.selected {
            0 => &mut self.tb_d4,
            1 => &mut self.tb_d6,
            2 => &mut self.tb_d8,
            3 => &mut self.tb_d10,
            4 => &mut self.tb_d20,
            5 => &mut self.tb_mod,
            _ => panic!("Could not find selected control")
        }
    }

    pub fn handle_key(&mut self, key: ::keyboard::Key) {
        self.selected_control_mut().handle_key(key);
    }

}

