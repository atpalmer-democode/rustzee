use std::fmt;
use crate::die::Die;
use crate::roll::{Roll, Keep};

pub struct TurnState {
    current: Roll,
    rolls_left: i32,
}

impl TurnState {
    pub const START_ROLLS: i32 = 3;

    pub fn has_rolls(&self) -> bool {
        return self.rolls_left > 0;
    }

    pub fn rolls_left(&self) -> i32 {
        return self.rolls_left;
    }

    pub fn roll() -> TurnState {
        return TurnState {
            current: Roll::roll(),
            rolls_left: Self::START_ROLLS - 1,
        };
    }

    pub fn reroll(&self, keep: Keep) -> TurnState {
        assert!(self.rolls_left > 0);
        return TurnState {
            current: Roll::reroll(&self.current, keep),
            rolls_left: self.rolls_left - 1,
        };
    }

    pub fn die_iter(&self) -> std::slice::Iter<Die> {
        return self.current.into_iter();
    }

    pub fn current(&self) -> &Roll {
        return &self.current;
    }
}

impl fmt::Display for TurnState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Roll: {}", self.current)
    }
}

