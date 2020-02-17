use std::fmt;
use crate::die::Die;
use crate::roll::{Roll, Keep};
use crate::scoring;

pub struct TurnState {
    current: Option<Roll>,
    roll_count: i32,
}

impl TurnState {
    pub fn new() -> TurnState {
        return TurnState {
            current: None,
            roll_count: 0,
        };
    }

    pub fn has_rolls(&self, allowed: i32) -> bool {
        return self.roll_count < allowed;
    }

    pub fn roll(&mut self) {
        self.current = Some(Roll::roll());
        self.roll_count = self.roll_count + 1;
    }

    pub fn reroll(&mut self, keep: Keep) {
        self.current = Some(Roll::reroll(&self.current.as_ref().unwrap(), keep));
        self.roll_count = self.roll_count + 1;
    }

    pub fn die_iter(&self) -> std::slice::Iter<Die> {
        return self.current.as_ref().unwrap().into_iter();
    }

    pub fn current(&self) -> &Roll {
        return self.current.as_ref().unwrap();
    }
}

impl fmt::Display for TurnState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self.current {
            Some(x) => display(x, f),
            None => write!(f, ""),
        };
    }
}

fn display(roll: &Roll, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "Roll: {}", roll)?;
    writeln!(f, "Score as 1s: {}", scoring::score_as(&roll, 1))?;
    writeln!(f, "Score as 2s: {}", scoring::score_as(&roll, 2))?;
    writeln!(f, "Score as 3s: {}", scoring::score_as(&roll, 3))?;
    writeln!(f, "Score as 4s: {}", scoring::score_as(&roll, 4))?;
    writeln!(f, "Score as 5s: {}", scoring::score_as(&roll, 5))?;
    writeln!(f, "Score as 6s: {}", scoring::score_as(&roll, 6))?;
    writeln!(f, "3 of a kind: {}", scoring::three_of_a_kind(&roll))?;
    writeln!(f, "4 of a kind: {}", scoring::four_of_a_kind(&roll))?;
    writeln!(f, "Full House: {}", scoring::full_house(&roll))?;
    writeln!(f, "Small Straight: {}", scoring::small_straight(&roll))?;
    writeln!(f, "Large Straight: {}", scoring::large_straight(&roll))?;
    writeln!(f, "Rustzee: {}", scoring::rustzee(&roll))?;
    writeln!(f, "Chance: {}", scoring::chance(&roll))
}

