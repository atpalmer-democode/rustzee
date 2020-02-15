use std::fmt::{Display, Formatter, Result};
#[path = "die.rs"]
mod die;
use self::die::Die;

pub struct Roll {
    dice: [Die; 5],
}

impl Roll {
    pub fn from(a: i32, b: i32, c: i32, d: i32, e: i32) -> Roll {
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

