use control::Control;
use rustbox::keyboard::Key;
use rustbox::RustBox;

pub struct ControlManager<'a> {
    rustbox: &'a RustBox,
    tb_d4: ::control::TextBox,
    tb_d6: ::control::TextBox,
    tb_d8: ::control::TextBox,
    tb_d10: ::control::TextBox,
    tb_d12: ::control::TextBox,
    tb_d20: ::control::TextBox,
    tb_mod: ::control::TextBox,
    ls_saved: ::control::ListSelect,
    ls_history: ::control::ListSelect,
    selected: usize,
    bottom_text: String,
}

impl<'a> ControlManager<'a> {
    pub fn initialize(rustbox: &'a RustBox) -> Self {
        let tb_d4 = ::control::TextBox::new(" d4", 3, true);
        let tb_d6 = ::control::TextBox::new(" d6", 3, true);
        let tb_d8 = ::control::TextBox::new(" d8", 3, true);
        let tb_d10 = ::control::TextBox::new("d10", 3, true);
        let tb_d12 = ::control::TextBox::new("d12", 3, true);
        let tb_d20 = ::control::TextBox::new("d20", 3, true);
        let tb_mod = ::control::TextBox::new("mod", 3, false);
        let ls_saved = ::control::ListSelect::new("Saved");
        let ls_history = ::control::ListSelect::new("History");

        let mut result = ControlManager {
            rustbox: rustbox,
            tb_d4: tb_d4,
            tb_d6: tb_d6,
            tb_d8: tb_d8,
            tb_d10: tb_d10,
            tb_d12: tb_d12,
            tb_d20: tb_d20,
            tb_mod: tb_mod,
            ls_saved: ls_saved,
            ls_history: ls_history,
            selected: 0,
            bottom_text: String::from(""),
        };

        result.tb_d4.set_selected(true);
        result
    }

    pub fn reposition(&mut self) {
        let mut top_y = self.rustbox.height() / 2 - 7;
        if top_y <= 0 {
            top_y = 2;
        }

        for (i, tb) in self.textboxes_mut().iter_mut().enumerate() {
            tb.set_location(1, top_y + i * 2);
        }

        self.ls_saved.set_location(self.rustbox.width() - 61, 1);
        self.ls_saved.set_size(30, self.rustbox.height() - 3);
        self.ls_history.set_location(self.rustbox.width() - 30, 1);
        self.ls_history.set_size(30, self.rustbox.height() - 3);
    }

    pub fn redraw(&self) {
        for tb in self.textboxes().iter() {
            tb.redraw(self.rustbox);
        }
        self.ls_history.redraw(self.rustbox);
        self.ls_saved.redraw(self.rustbox);
        self.rustbox.print(
            0,
            self.rustbox.height() - 1,
            ::rustbox::RB_NORMAL,
            ::rustbox::Color::White,
            ::rustbox::Color::Black,
            &::std::iter::repeat(' ')
                .take(self.rustbox.width())
                .collect::<String>(),
        );
        self.rustbox.print(
            0,
            self.rustbox.height() - 1,
            ::rustbox::RB_NORMAL,
            ::rustbox::Color::White,
            ::rustbox::Color::Black,
            &self.bottom_text,
        );
        self.rustbox.present();
    }

    pub fn select_next(&mut self, skip_lists: bool) {
        self.selected_control_mut().set_selected(false);
        self.selected += 1;
        if self.selected == 7 && skip_lists {
            self.selected = 0;
        }
        if self.selected == 7 && !self.ls_saved.has_items() {
            self.selected += 1;
        }
        if self.selected == 8 && !self.ls_history.has_items() {
            self.selected += 1;
        }
        if self.selected == 9 {
            self.selected = 0;
        }
        self.selected_control_mut().set_selected(true);
    }

    pub fn select_prev(&mut self) {
        self.selected_control_mut().set_selected(false);

        if self.selected == 0 {
            self.selected = 6;
        } else {
            self.selected -= 1;
        }
        if self.selected == 8 && !self.ls_history.has_items() {
            self.selected -= 1;
        }
        if self.selected == 7 && !self.ls_saved.has_items() {
            self.selected -= 1;
        }
        self.selected_control_mut().set_selected(true);
    }

    fn selected_control_mut(&mut self) -> &mut dyn Control {
        match self.selected {
            0 => &mut self.tb_d4,
            1 => &mut self.tb_d6,
            2 => &mut self.tb_d8,
            3 => &mut self.tb_d10,
            4 => &mut self.tb_d12,
            5 => &mut self.tb_d20,
            6 => &mut self.tb_mod,
            7 => &mut self.ls_saved,
            8 => &mut self.ls_history,
            _ => panic!("Could not find selected control"),
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        match key {
            Key::Enter => self.roll_dice(),
            Key::Tab => self.select_next(false),
            Key::Down => {
                if self.selected <= 6 {
                    self.select_next(true);
                } else {
                    self.selected_control_mut().handle_key(key);
                }
            }
            Key::Up => {
                if self.selected <= 6 {
                    self.select_prev();
                } else {
                    self.selected_control_mut().handle_key(key);
                }
            }
            Key::Char('l') => {
                if self.selected == 7 {
                    let dice = self.ls_saved.selected_item().unwrap();
                    self.load_dice(dice);
                } else if self.selected == 8 {
                    let dice = self.ls_history.selected_item().unwrap();
                    self.load_dice(dice);
                }
            }
            Key::Char('s') => {
                if !self.textboxes().iter().all(|tb| tb.get_isize() == 0) {
                    let dice = self.current_dice();
                    self.ls_saved.add_item(dice);
                }
            }
            Key::Char('c') => self.clear_dice(),
            Key::Char('C') => {
                self.clear_dice();
                self.clear_lists();
            }
            _ => (self.selected_control_mut().handle_key(key)),
        }
    }

    fn load_dice(&mut self, dice: ::dice::Dice) {
        self.tb_d4.set_isize(dice.d4s as isize);
        self.tb_d6.set_isize(dice.d6s as isize);
        self.tb_d8.set_isize(dice.d8s as isize);
        self.tb_d10.set_isize(dice.d10s as isize);
        self.tb_d12.set_isize(dice.d12s as isize);
        self.tb_d20.set_isize(dice.d20s as isize);
        self.tb_mod.set_isize(dice.modifier as isize);
    }

    fn current_dice(&self) -> ::dice::Dice {
        ::dice::Dice {
            d4s: self.tb_d4.get_isize() as usize,
            d6s: self.tb_d6.get_isize() as usize,
            d8s: self.tb_d8.get_isize() as usize,
            d10s: self.tb_d10.get_isize() as usize,
            d12s: self.tb_d12.get_isize() as usize,
            d20s: self.tb_d20.get_isize() as usize,
            modifier: self.tb_mod.get_isize(),
        }
    }

    fn roll_dice(&mut self) {
        if !self.textboxes().iter().all(|tb| tb.get_isize() == 0) {
            let dice = self.current_dice();

            self.ls_history.add_item(dice);

            let dice_roll = ::dice_roll::DiceRoll::new(dice);
            self.bottom_text = format!("{:?}", dice_roll);
        }
    }

    fn clear_dice(&mut self) {
        for tb in self.textboxes_mut().iter_mut() {
            tb.clear_data();
        }
    }

    fn clear_lists(&mut self) {
        self.ls_history.clear_data();
        self.ls_saved.clear_data();
    }

    fn textboxes_mut(&mut self) -> Vec<&mut ::control::TextBox> {
        vec![
            &mut self.tb_d4,
            &mut self.tb_d6,
            &mut self.tb_d8,
            &mut self.tb_d10,
            &mut self.tb_d12,
            &mut self.tb_d20,
            &mut self.tb_mod,
        ]
    }

    fn textboxes(&self) -> Vec<&::control::TextBox> {
        vec![
            &self.tb_d4,
            &self.tb_d6,
            &self.tb_d8,
            &self.tb_d10,
            &self.tb_d12,
            &self.tb_d20,
            &self.tb_mod,
        ]
    }
}
