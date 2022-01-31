use std::fmt::{Display, Formatter, Result};
use crate::die::Die;
use super::keep::Keep;
use super::value_counts::ValueCounts;

pub struct Roll {
    dice: [Die; 5],
    counts: ValueCounts,
}

impl From<[Die; 5]> for Roll {
    fn from(value: [Die; 5]) -> Roll {
        let counts = ValueCounts::new(&value);
        return Roll {
            dice: value,
            counts: counts,
        };
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

    pub fn die_value_total(&self, value: i32) -> i32 {
        return self.counts.die_value_total(value);
    }

    pub fn has_exact(&self, kind: i32) -> bool {
        return self.counts.has_exact(kind);
    }

    pub fn has_kind(&self, kind: i32) -> bool {
        return self.counts.has_kind(kind);
    }

    pub fn straight_len(&self) -> i32 {
        return self.counts.straight_len();
    }

    pub fn total(&self) -> i32 {
        return self.counts.total();
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

