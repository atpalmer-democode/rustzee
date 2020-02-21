use std::fmt;
use crate::die::Die;
use crate::roll::{Roll, Keep};

pub struct TurnState {
    current: Option<Roll>,
    roll_count: i32,
}

impl TurnState {
    pub const MAX_ROLLS: i32 = 3;

    pub fn new() -> TurnState {
        return TurnState {
            current: None,
            roll_count: 0,
        };
    }

    pub fn has_rolls(&self) -> bool {
        return self.roll_count < Self::MAX_ROLLS;
    }

    pub fn rolls_left(&self) -> i32 {
        let result = Self::MAX_ROLLS - self.roll_count;
        assert!(result >= 0);
        return result;
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
            Some(roll) => writeln!(f, "Roll: {}", roll),
            None => write!(f, ""),
        };
    }
}

