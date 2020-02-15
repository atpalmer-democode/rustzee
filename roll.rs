use std::fmt::{Display, Formatter, Result};
#[path = "die.rs"]
mod die;
use self::die::Die;

pub struct Roll {
    dice: [Die; 5],
}

impl From<[Die; 5]> for Roll {
    fn from(value: [Die; 5]) -> Roll {
        return Roll { dice: value };
    }
}

impl Roll {
    pub fn of(a: Die, b: Die, c: Die, d: Die, e: Die) -> Roll {
        return Roll::from([a, b, c, d, e]);
    }

    pub fn roll() -> Roll {
        return Roll::of(
            Die::roll(),
            Die::roll(),
            Die::roll(),
            Die::roll(),
            Die::roll(),
        );
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{:?}", self.dice);
    }
}

