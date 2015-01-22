
use rustbox::{ RustBox };
use ::control::Control;
use ::keyboard::Key;


pub struct Controls<'a> {
    rustbox: &'a RustBox,
    tb_d4: ::control::TextBox,
    tb_d6: ::control::TextBox,
    tb_d8: ::control::TextBox,
    tb_d10: ::control::TextBox,
    tb_d20: ::control::TextBox,
    tb_mod: ::control::TextBox,
    selected: usize,
    bottom_text: String
}

impl<'a> Controls<'a> {

    pub fn initialize(rustbox: &'a RustBox) -> Self {
        let tb_d4 = ::control::TextBox::new(" d4", 3);
        let tb_d6 = ::control::TextBox::new(" d6", 3);
        let tb_d8 = ::control::TextBox::new(" d8", 3);
        let tb_d10 = ::control::TextBox::new("d10", 3);
        let tb_d20 = ::control::TextBox::new("d20", 3);
        let tb_mod = ::control::TextBox::new("mod", 3);

        let mut result = Controls {
            rustbox: rustbox,
            tb_d4: tb_d4,
            tb_d6: tb_d6,
            tb_d8: tb_d8,
            tb_d10: tb_d10,
            tb_d20: tb_d20,
            tb_mod: tb_mod,
            selected: 0,
            bottom_text: String::from_str("")
        };

        result.tb_d4.set_selected(true);
        result
    }

    pub fn reposition(&mut self) {
        let mut top_y = self.rustbox.height() / 2 - 6;
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

    pub fn redraw(&self) {
        self.tb_d4.redraw(self.rustbox);
        self.tb_d6.redraw(self.rustbox);
        self.tb_d8.redraw(self.rustbox);
        self.tb_d10.redraw(self.rustbox);
        self.tb_d20.redraw(self.rustbox);
        self.tb_mod.redraw(self.rustbox);
        self.rustbox.print(0, 
                           self.rustbox.height() - 1, 
                           ::rustbox::RB_NORMAL, 
                           ::rustbox::Color::White, 
                           ::rustbox::Color::Black, 
                           ::std::iter::repeat(' ').take(self.rustbox.width()).collect::<String>().as_slice());
        self.rustbox.print(0, 
                           self.rustbox.height() - 1, 
                           ::rustbox::RB_NORMAL, 
                           ::rustbox::Color::White, 
                           ::rustbox::Color::Black, 
                           self.bottom_text.as_slice());
        self.rustbox.present();
    }

    pub fn select_next(&mut self) {
        self.selected_control_mut().set_selected(false);
        self.selected += 1;
        if self.selected == 6 { self.selected = 0; }
        self.selected_control_mut().set_selected(true);
    }

    pub fn select_prev(&mut self) {
        self.selected_control_mut().set_selected(false);
        if self.selected == 0 {
            self.selected = 5;
        } else {
            self.selected -= 1;
        }
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
        match key {
            Key::Enter => self.roll_dice(),
            Key::Tab | Key::Down  => self.select_next(),
            Key::Up => self.select_prev(),
            _ => (self.selected_control_mut().handle_key(key))
        }
    }

    fn roll_dice(&mut self) {
        let dice = ::dice::Dice {
            d4s: self.tb_d4.get_isize() as usize,
            d6s: self.tb_d6.get_isize() as usize,
            d8s: self.tb_d8.get_isize() as usize,
            d10s: self.tb_d10.get_isize() as usize,
            d20s: self.tb_d20.get_isize() as usize,
            modifier: self.tb_mod.get_isize(),
        };

        let dice_roll = ::dice_roll::DiceRoll::new(dice);
        self.bottom_text = format!("{:?}", dice_roll);
    }

}

pub trait ControlCallback {
    fn load_saved(&self);
    fn roll_dice(&self);
    fn save_new(&self);
}

impl<'a> ControlCallback for Controls<'a> {
    fn load_saved(&self) {

    }
    fn roll_dice(&self){
    }
    fn save_new(&self){
    }
}
