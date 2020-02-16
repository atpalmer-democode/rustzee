use std::fmt::{Display, Formatter, Result};
use crate::die::Die;
use super::roll::Roll;

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

    pub fn at_or_new(&self, roll: &Roll, i: usize) -> Die {
        return match self.dice[i] {
            true => Die::from(roll.at(i)),
            false => Die::roll(),
        };
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

