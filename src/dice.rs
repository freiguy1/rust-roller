use std::fmt::Show;

pub struct Dice {
    pub d4s: usize,
    pub d6s: usize,
    pub d8s: usize,
    pub d10s: usize,
    pub d20s: usize,
    pub modifier: isize
}

impl Show for Dice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut buff = String::new();
        if self.d4s != 0 {
            buff.push_str(format!("{}d4 ", self.d4s).as_slice());
        }
        if self.d6s != 0 {
            buff.push_str(format!("{}d6 ", self.d6s).as_slice());
        }
        if self.d8s != 0 {
            buff.push_str(format!("{}d8 ", self.d8s).as_slice());
        }
        if self.d10s != 0 {
            buff.push_str(format!("{}d10 ", self.d10s).as_slice());
        }
        if self.d20s != 0 {
            buff.push_str(format!("{}d20 ", self.d20s).as_slice());
        }
        if self.modifier > 0 {
            buff.push_str(format!("+{}", self.modifier).as_slice());
        }
        if self.modifier < 0 {
            buff.push_str(format!("-{}", self.modifier).as_slice());
        }
        write!(f, "{}", buff.as_slice().trim())
    }
}
