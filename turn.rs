use crate::roll::{Roll, Keep};

pub struct TurnState {
    roll: Roll,
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
            roll: Roll::roll(),
            rolls_left: Self::START_ROLLS - 1,
        };
    }

    pub fn reroll(&self, keep: Keep) -> TurnState {
        assert!(self.rolls_left > 0);
        return TurnState {
            roll: Roll::reroll(&self.roll, keep),
            rolls_left: self.rolls_left - 1,
        };
    }

    pub fn current(&self) -> &Roll {
        return &self.roll;
    }
}

