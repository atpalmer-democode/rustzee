use std::fmt::{Display, Formatter, Result};
#[path = "die.rs"]
mod die;
use self::die::Die;

pub struct Keep {
    dice: [bool; 5],
}

impl Keep {
    pub fn dice(a: bool, b: bool, c: bool, d: bool, e: bool) -> Keep {
        return Keep {
            dice: [a, b, c, d, e],
        };
    }

    fn keeping(&self) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, keeping) in self.dice.iter().enumerate() {
            if *keeping {
                result.push(i);
            }
        }
        return result;
    }
}

impl Display for Keep {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let indexes = self.keeping();
        writeln!(f, "Keeping {}: [", indexes.len())?;
        for i in indexes {
            writeln!(f, "  {},", i)?;
        }
        return writeln!(f, "]");
    }
}

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

