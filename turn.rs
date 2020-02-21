use std::fmt;
use crate::die::Die;
use crate::roll::{Roll, Keep};

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
            Some(roll) => writeln!(f, "Roll: {}", roll),
            None => write!(f, ""),
        };
    }
}

