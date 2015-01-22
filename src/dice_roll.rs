use std::fmt::Show;
use std::rand::Rng;
use ::dice::Dice;
use std::num::SignedInt;


pub struct DiceRoll {
    dice: Dice,
    d4_result: Vec<usize>,
    d6_result: Vec<usize>,
    d8_result: Vec<usize>,
    d10_result: Vec<usize>,
    d20_result: Vec<usize>,
}

impl DiceRoll {
    pub fn new(dice: Dice) -> Self {
        let mut rng = ::std::rand::thread_rng();
        let mut result = DiceRoll {
            dice: dice,
            d4_result: Vec::new(),
            d6_result: Vec::new(),
            d8_result: Vec::new(),
            d10_result: Vec::new(),
            d20_result: Vec::new()
        };

        // Roll d4s
        for _ in range(0, result.dice.d4s) {
            result.d4_result.push(rng.gen_range(1, 5));
        }
        // Roll d6s
        for _ in range(0, result.dice.d6s) {
            result.d6_result.push(rng.gen_range(1, 7));
        }
        // Roll d8s
        for _ in range(0, result.dice.d8s) {
            result.d8_result.push(rng.gen_range(1, 9));
        }
        // Roll d10s
        for _ in range(0, result.dice.d10s) {
            result.d10_result.push(rng.gen_range(1, 11));
        }
        // Roll d20s
        for _ in range(0, result.dice.d20s) {
            result.d20_result.push(rng.gen_range(1, 21));
        }

        result
    }

    pub fn result(&self) -> isize {
        let mut result = self.dice.modifier;
        result += DiceRoll::sum(&self.d4_result) as isize;
        result += DiceRoll::sum(&self.d6_result) as isize;
        result += DiceRoll::sum(&self.d8_result) as isize;
        result += DiceRoll::sum(&self.d10_result) as isize;
        result += DiceRoll::sum(&self.d20_result) as isize;
        result
    }
    
    fn sum(list: &Vec<usize>) -> usize {
        let mut result = 0us;
        for item in list.iter() {
            result += *item;
        }
        result
    }

    fn format(list: &Vec<usize>, name: &str) -> String {
        let mut buff = String::new();
        if list.len() == 1 {
            buff.push_str(format!("1{}({}) ", name, list[0]).as_slice());
        } else if list.len() > 1 {
            buff.push_str(format!("{}{}(", list.len(), name).as_slice());
            for (i, value) in list.iter().enumerate() {
                buff.push_str(format!("{}", value).as_slice());
                if i != list.len() - 1 {
                    buff.push_str("+");
                }
            }
            buff.push_str(format!("={}) ", DiceRoll::sum(list)).as_slice());
        }
        buff
    }
}

impl Show for DiceRoll {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut buff = format!("{} = ", self.result());
        buff.push_str(DiceRoll::format(&self.d4_result, "d4").as_slice());
        buff.push_str(DiceRoll::format(&self.d6_result, "d6").as_slice());
        buff.push_str(DiceRoll::format(&self.d8_result, "d8").as_slice());
        buff.push_str(DiceRoll::format(&self.d10_result, "d10").as_slice());
        buff.push_str(DiceRoll::format(&self.d20_result, "d20").as_slice());
        if self.dice.modifier > 0 {
            buff.push_str(format!("+ {}", self.dice.modifier).as_slice());
        } else if self.dice.modifier < 0 {
            buff.push_str(format!("- {}", self.dice.modifier.abs()).as_slice());
        }
        write!(f, "{}", buff.as_slice().trim())
    }
}
