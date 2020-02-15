use std::fmt::{Display, Formatter, Result};
mod die;
use die::Die;

struct Roll {
    dice: [Die; 5],
}

impl Roll {
    fn from(a: i32, b: i32, c: i32, d: i32, e: i32) -> Roll {
        return Roll {
            dice: [
                Die::from(a),
                Die::from(b),
                Die::from(c),
                Die::from(d),
                Die::from(e),
            ],
        };
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{:?}", self.dice);
    }
}

fn main() {
    let roll = Roll::from(6, 5, 4, 3, 2);
    let die = Die::from(6);
    println!("Dice: {}", roll);
    println!("Die: {}", die);
}
