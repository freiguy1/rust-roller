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
    d12_result: Vec<usize>,
    d20_result: Vec<usize>,
}

impl DiceRoll {
    pub fn new(dice: Dice) -> Self {
        let d4_result = DiceRoll::generate_rolls(4, dice.d4s);
        let d6_result = DiceRoll::generate_rolls(6, dice.d6s);
        let d8_result = DiceRoll::generate_rolls(8, dice.d8s);
        let d10_result = DiceRoll::generate_rolls(10, dice.d10s);
        let d12_result = DiceRoll::generate_rolls(12, dice.d12s);
        let d20_result = DiceRoll::generate_rolls(20, dice.d20s);
        DiceRoll {
            dice: dice,
            d4_result: d4_result,
            d6_result: d6_result,
            d8_result: d8_result,
            d10_result: d10_result,
            d12_result: d12_result,
            d20_result: d20_result
        }
    }

    fn generate_rolls(dice_max: usize, number: usize) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut rng = ::std::rand::thread_rng();
        for _ in range(0, number) {
            result.push(rng.gen_range(0, dice_max) + 1);
        }
        result
    }

    pub fn result(&self) -> isize {
        let mut result = self.dice.modifier;
        result += DiceRoll::sum(&self.d4_result) as isize;
        result += DiceRoll::sum(&self.d6_result) as isize;
        result += DiceRoll::sum(&self.d8_result) as isize;
        result += DiceRoll::sum(&self.d10_result) as isize;
        result += DiceRoll::sum(&self.d12_result) as isize;
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
        buff.push_str(DiceRoll::format(&self.d12_result, "d12").as_slice());
        buff.push_str(DiceRoll::format(&self.d20_result, "d20").as_slice());
        if self.dice.modifier > 0 {
            buff.push_str(format!("+ {}", self.dice.modifier).as_slice());
        } else if self.dice.modifier < 0 {
            buff.push_str(format!("- {}", self.dice.modifier.abs()).as_slice());
        }
        write!(f, "{}", buff.as_slice().trim())
    }
}
