use std::fmt::{Display, Formatter, Result};
use crate::die::Die;
use super::keep::Keep;

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

    pub fn at(&self, i: usize) -> &Die {
        return &self.dice[i];
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

