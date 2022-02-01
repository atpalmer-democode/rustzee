use crate::roll::Roll;

pub trait ScoreCardEntry {
    fn text(&self) -> &str;
    fn clone(&self) -> Box<dyn ScoreCardEntry>;
    fn get(&self) -> Option<i32>;
    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32>;
}

#[derive(Default, Clone)]
pub struct EntryBase(Option<i32>);

impl EntryBase {
    pub fn get(&self) -> Option<i32> {
        return self.0;
    }

    pub fn try_set(&mut self, val: i32) -> Result<i32, i32> {
        return match self.0 {
            None => { self.0 = Some(val); Ok(val) },
            Some(existing_val) => Err(existing_val),
        }
    }
}

#[derive(Default, Clone)]
pub struct Aces(EntryBase);

#[derive(Default, Clone)]
pub struct Twos(EntryBase);

#[derive(Default, Clone)]
pub struct Threes(EntryBase);

#[derive(Default, Clone)]
pub struct Fours(EntryBase);

#[derive(Default, Clone)]
pub struct Fives(EntryBase);

#[derive(Default, Clone)]
pub struct Sixes(EntryBase);

impl ScoreCardEntry for Aces {
    fn text(&self) -> &str {
        return "Aces";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(1);
        return self.0.try_set(result);
    }
}

impl ScoreCardEntry for Twos {
    fn text(&self) -> &str {
        return "Twos";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(2);
        return self.0.try_set(result);
    }
}

impl ScoreCardEntry for Threes {
    fn text(&self) -> &str {
        return "Threes";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(3);
        return self.0.try_set(result);
    }
}

impl ScoreCardEntry for Fours {
    fn text(&self) -> &str {
        return "Fours";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(4);
        return self.0.try_set(result);
    }
}

impl ScoreCardEntry for Fives {
    fn text(&self) -> &str {
        return "Fives";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(5);
        return self.0.try_set(result);
    }
}

impl ScoreCardEntry for Sixes {
    fn text(&self) -> &str {
        return "Sixes";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(6);
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct ThreeOfAKind(EntryBase);

impl ScoreCardEntry for ThreeOfAKind {
    fn text(&self) -> &str {
        return "3 of a Kind";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.has_kind(3) {
            true => roll.total(),
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct FourOfAKind(EntryBase);

impl ScoreCardEntry for FourOfAKind {
    fn text(&self) -> &str {
        return "4 of a Kind";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.has_kind(4) {
            true => roll.total(),
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct FullHouse(EntryBase);

impl ScoreCardEntry for FullHouse {
    fn text(&self) -> &str {
        return "Full House";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.has_exact(3) && roll.has_exact(2) {
            true => 25,
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct SmallStraight(EntryBase);

impl ScoreCardEntry for SmallStraight {
    fn text(&self) -> &str {
        return "Sm. Straight";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.straight_len() >= 4 {
            true => 30,
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct LargeStraight(EntryBase);

impl ScoreCardEntry for LargeStraight {
    fn text(&self) -> &str {
        return "Lg. Straight";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.straight_len() == 5 {
            true => 40,
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct Chance(EntryBase);

impl ScoreCardEntry for Chance {
    fn text(&self) -> &str {
        return "Chance";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.total();
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct Rustzee {
    first: EntryBase,
    bonus: i32,
}

impl ScoreCardEntry for Rustzee {
    fn text(&self) -> &str {
        return "Rustzee";
    }

    fn clone(&self) -> Box<dyn ScoreCardEntry> {
        return Box::new(Clone::clone(self));
    }

    fn get(&self) -> Option<i32> {
        return self.first.get().and_then(|v| Some(v + self.bonus));
    }

    fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        if !roll.has_kind(5) {
            return self.first.try_set(0);
        }
        return match self.first.get() {
            None => self.first.try_set(50),
            Some(50) => {
                self.bonus += 100;
                return Ok(50 + self.bonus);
            },
            Some(v) => panic!("First Rustzee has value: {}", v)
        };
    }
}

