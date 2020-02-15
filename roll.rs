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

impl From<[i32; 5]> for Roll {
    fn from(value: [i32; 5]) -> Roll {
        let dice: [Die; 5] = [
            value[0].into(),
            value[1].into(),
            value[2].into(),
            value[3].into(),
            value[4].into(),
        ];
        return Roll::from(dice);
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

