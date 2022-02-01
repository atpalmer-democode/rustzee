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

pub struct ScoreCard {
    entries: Vec<Box<dyn ScoreCardEntry>>
}

impl ScoreCard {
    pub fn new() -> ScoreCard {
        return ScoreCard {
            entries: vec![
                Box::new(Aces::default()),
                Box::new(Twos::default()),
                Box::new(Threes::default()),
                Box::new(Fours::default()),
                Box::new(Fives::default()),
                Box::new(Sixes::default()),
                Box::new(ThreeOfAKind::default()),
                Box::new(FourOfAKind::default()),
                Box::new(FullHouse::default()),
                Box::new(SmallStraight::default()),
                Box::new(LargeStraight::default()),
                Box::new(Rustzee::default()),
                Box::new(Chance::default()),
            ]
        };
    }

    fn clone(&self) -> ScoreCard {
        let entries = self.entries.iter()
            .map(|x| (*x).clone())
            .collect();
        return ScoreCard { entries };
    }

    fn top_subtotal(&self) -> i32 {
        return self.entries[0..6].iter()
            .filter_map(|x| x.get())
            .sum();
    }

    fn top_bonus(&self) -> i32 {
        return match self.top_subtotal() >= 63 {
            true => 35,
            false => 0,
        };
    }

    fn top_total(&self) -> i32 {
        return self.top_subtotal() + self.top_bonus();
    }

    fn bottom_total(&self) -> i32 {
        return self.entries[6..].iter()
            .filter_map(|x| x.get())
            .sum();
    }

    pub fn total(&self) -> i32 {
        return self.top_total() + self.bottom_total();
    }

    pub fn options(&self, roll: &Roll) -> Vec<(usize, &str, i32, i32)> {
        return self.entries.iter().enumerate()
            .filter_map(|(i, e)| {
                let mut clone = self.clone();  // TODO: avoid full clones
                return clone.score_by_func_index(roll, i).and_then(|value| {
                    Some((i + 1, (*e).text(), value, clone.total()))
                });
            }).collect();
    }

    fn score_by_func_index(&mut self, roll: &Roll, index: usize) -> Option<i32> {
        return self.entries[index].try_score(roll).ok();
    }

    pub fn score_by_option(&mut self, roll: &Roll, choice: usize) -> Option<i32> {
        let index = choice.checked_sub(1)?;
        return self.score_by_func_index(roll, index);
    }
}

