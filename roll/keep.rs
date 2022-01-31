use std::fmt::{Display, Formatter, Result};
use crate::die::Die;
use super::roll::Roll;

pub struct Keep {
    dice: [bool; 5],
}

impl Keep {
    pub fn new(keepers: [bool; 5]) -> Keep {
        return Keep { dice: keepers };
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
        write!(f, "Keeping {}: ", indexes.len())?;
        if indexes.len() == 0 {
            return writeln!(f, "[ None ]");
        }
        let mut iter = indexes.into_iter();
        write!(f, "[ Die #{}", iter.next().unwrap() + 1)?;
        for i in iter {
            write!(f, ", #{}", i + 1)?;
        }
        return writeln!(f, " ]");
    }
}

