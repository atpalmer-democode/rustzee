use std::fmt::{Display, Formatter, Result};
use crate::die::Die;

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

    fn at_or_new(&self, roll: &Roll, i: usize) -> Die {
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

    pub fn reroll(&self, keep: Keep) -> Roll {
        let new_dice: [Die; 5] = [
            keep.at_or_new(&self, 0),
            keep.at_or_new(&self, 1),
            keep.at_or_new(&self, 2),
            keep.at_or_new(&self, 3),
            keep.at_or_new(&self, 4),
        ];
        return Roll::from(new_dice);
    }

    fn at(&self, i: usize) -> &Die {
        return &self.dice[i];
    }

    pub fn count_values(&self, value: i32) -> i32 {
        let mut result = 0;
        for die in &self.dice {
            if die.value() == value {
                result += 1;
            }
        }
        return result;
    }
}

impl<'a> IntoIterator for &'a Roll {
    type Item = &'a Die;
    type IntoIter = std::slice::Iter<'a, Die>;

    fn into_iter(self) -> Self::IntoIter {
        return self.dice.iter();
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{:?}", self.dice);
    }
}

