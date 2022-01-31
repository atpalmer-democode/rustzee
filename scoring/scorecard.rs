use crate::roll::Roll;
use super::scorefunc;

// SCORE_OPT_TEXT and SCORE_OPT_FUNC are parallel arrays with corresponding elements.
// Index of SCORE_OPT_TEXT is used to lookup function in SCORE_OPT_FUNC.

type ScoreOpts<T> = [T; 13];

const SCORE_OPT_TEXT: ScoreOpts<&str> = [
    "Aces",
    "Twos",
    "Threes",
    "Fours",
    "Fives",
    "Sixes",
    "3 of a Kind",
    "4 of a Kind",
    "Full House",
    "Sm. Straight",
    "Lg. Stright",
    "Rustzee",
    "Chance",
];

const SCORE_OPT_FUNC: ScoreOpts<scorefunc::ScoreFunc> = [
    scorefunc::score_aces,
    scorefunc::score_twos,
    scorefunc::score_threes,
    scorefunc::score_fours,
    scorefunc::score_fives,
    scorefunc::score_sixes,
    scorefunc::score_three_of_a_kind,
    scorefunc::score_four_of_a_kind,
    scorefunc::score_full_house,
    scorefunc::score_small_straight,
    scorefunc::score_large_straight,
    scorefunc::score_rustzee,
    scorefunc::score_chance,
];

#[derive(Default, Clone)]
pub struct ScoreCardEntry(Option<i32>);

impl ScoreCardEntry {
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
pub struct Aces(ScoreCardEntry);

#[derive(Default, Clone)]
pub struct Twos(ScoreCardEntry);

#[derive(Default, Clone)]
pub struct Threes(ScoreCardEntry);

#[derive(Default, Clone)]
pub struct Fours(ScoreCardEntry);

#[derive(Default, Clone)]
pub struct Fives(ScoreCardEntry);

#[derive(Default, Clone)]
pub struct Sixes(ScoreCardEntry);

impl Aces {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(1);
        return self.0.try_set(result);
    }
}

impl Twos {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(2);
        return self.0.try_set(result);
    }
}

impl Threes {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(3);
        return self.0.try_set(result);
    }
}

impl Fours {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(4);
        return self.0.try_set(result);
    }
}

impl Fives {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(5);
        return self.0.try_set(result);
    }
}

impl Sixes {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.die_value_total(6);
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct ThreeOfAKind(ScoreCardEntry);

impl ThreeOfAKind {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.has_kind(3) {
            true => roll.total(),
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct FourOfAKind(ScoreCardEntry);

impl FourOfAKind {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.has_kind(4) {
            true => roll.total(),
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct FullHouse(ScoreCardEntry);

impl FullHouse {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.has_exact(3) && roll.has_exact(2) {
            true => 25,
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct SmallStraight(ScoreCardEntry);

impl SmallStraight {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.straight_len() >= 4 {
            true => 30,
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct LargeStraight(ScoreCardEntry);

impl LargeStraight {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = match roll.straight_len() == 5 {
            true => 40,
            false => 0,
        };
        return self.0.try_set(result);
    }
}

#[derive(Default, Clone)]
pub struct Chance(ScoreCardEntry);

impl Chance {
    pub fn get(&self) -> Option<i32> {
        return self.0.get();
    }

    pub fn try_score(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.total();
        return self.0.try_set(result);
    }
}


#[derive(Default, Clone)]
pub struct ScoreCard {
    pub(crate) aces: Aces,
    pub(crate) twos: Twos,
    pub(crate) threes: Threes,
    pub(crate) fours: Fours,
    pub(crate) fives: Fives,
    pub(crate) sixes: Sixes,
    pub(crate) three_of_a_kind: ThreeOfAKind,
    pub(crate) four_of_a_kind: FourOfAKind,
    pub(crate) full_house: FullHouse,
    pub(crate) small_straight: SmallStraight,
    pub(crate) large_straight: LargeStraight,
    pub(crate) rustzee: ScoreCardEntry,
    pub(crate) chance: Chance,
    pub(crate) rustzee_bonus: i32,
}

impl ScoreCard {
    pub fn new() -> ScoreCard {
        return ScoreCard::default();
    }

    fn top_subtotal(&self) -> i32 {
        let items = [
            self.aces.get(),
            self.twos.get(),
            self.threes.get(),
            self.fours.get(),
            self.fives.get(),
            self.sixes.get(),
        ];
        return items.iter().filter_map(|x| *x).sum();
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
        let items = [
            self.three_of_a_kind.get(),
            self.four_of_a_kind.get(),
            self.full_house.get(),
            self.small_straight.get(),
            self.large_straight.get(),
            self.rustzee.get(),
            self.chance.get(),
            Some(self.rustzee_bonus),
        ];
        return items.iter().filter_map(|x| *x).sum();
    }

    pub fn total(&self) -> i32 {
        return self.top_total() + self.bottom_total();
    }

    pub fn options(&self, roll: &Roll) -> Vec<(usize, &str, i32, i32)> {
        return SCORE_OPT_TEXT.iter().enumerate()
            .filter_map(|(i, text)| {
                let mut clone = self.clone();
                return clone.score_by_func_index(roll, i).and_then(|value| {
                    Some((i + 1, *text, value, clone.total()))
                });
            }).collect();
    }

    fn score_by_func_index(&mut self, roll: &Roll, index: usize) -> Option<i32> {
        let func = SCORE_OPT_FUNC.get(index)?;
        return func(self, &roll).ok();
    }

    pub fn score_by_option(&mut self, roll: &Roll, choice: usize) -> Option<i32> {
        let index = choice.checked_sub(1)?;
        return self.score_by_func_index(roll, index);
    }
}

