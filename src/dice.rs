use std::fmt::Debug;

#[derive(PartialEq, Clone, Copy)]
pub struct Dice {
    pub d4s: usize,
    pub d6s: usize,
    pub d8s: usize,
    pub d10s: usize,
    pub d12s: usize,
    pub d20s: usize,
    pub modifier: isize,
}

impl Debug for Dice {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut buff = String::new();
        if self.d4s != 0 {
            buff.push_str(&format!("{}d4 ", self.d4s));
        }
        if self.d6s != 0 {
            buff.push_str(&format!("{}d6 ", self.d6s));
        }
        if self.d8s != 0 {
            buff.push_str(&format!("{}d8 ", self.d8s));
        }
        if self.d10s != 0 {
            buff.push_str(&format!("{}d10 ", self.d10s));
        }
        if self.d12s != 0 {
            buff.push_str(&format!("{}d12 ", self.d12s));
        }
        if self.d20s != 0 {
            buff.push_str(&format!("{}d20 ", self.d20s));
        }
        if self.modifier > 0 {
            buff.push_str(&format!("+{}", self.modifier));
        }
        if self.modifier < 0 {
            buff.push_str(&format!("-{}", self.modifier.abs()));
        }
        write!(f, "{}", (&buff).trim())
    }
}
