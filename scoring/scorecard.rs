use crate::roll::Roll;
use super::value_counts::ValueCounts;

// SCORE_OPT_TEXT and SCORE_OPT_FUNC are parallel arrays with corresponding elements.
// Index of SCORE_OPT_TEXT is used to lookup function in SCORE_OPT_FUNC.

const SCORE_OPT_TEXT: [&str; 13] = [
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

const SCORE_OPT_FUNC: [for<'sc, 'r> fn(&'sc mut ScoreCard, &'r ValueCounts) -> Result<i32, i32>; 13] = [
    ScoreCard::score_aces,
    ScoreCard::score_twos,
    ScoreCard::score_threes,
    ScoreCard::score_fours,
    ScoreCard::score_fives,
    ScoreCard::score_sixes,
    ScoreCard::score_three_of_a_kind,
    ScoreCard::score_four_of_a_kind,
    ScoreCard::score_full_house,
    ScoreCard::score_small_straight,
    ScoreCard::score_large_straight,
    ScoreCard::score_rustzee,
    ScoreCard::score_chance,
];

#[derive(Default, Clone)]
pub struct ScoreCard {
    aces: Option<i32>,
    twos: Option<i32>,
    threes: Option<i32>,
    fours: Option<i32>,
    fives: Option<i32>,
    sixes: Option<i32>,
    three_of_a_kind: Option<i32>,
    four_of_a_kind: Option<i32>,
    full_house: Option<i32>,
    small_straight: Option<i32>,
    large_straight: Option<i32>,
    rustzee: Option<i32>,
    chance: Option<i32>,
    rustzee_bonus: i32,
}

impl ScoreCard {
    pub fn new() -> ScoreCard {
        return ScoreCard::default();
    }

    fn top_subtotal(&self) -> i32 {
        let items = [
            self.aces,
            self.twos,
            self.threes,
            self.fours,
            self.fives,
            self.sixes,
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
            self.three_of_a_kind,
            self.four_of_a_kind,
            self.full_house,
            self.small_straight,
            self.large_straight,
            self.rustzee,
            self.chance,
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
        let counts = ValueCounts::from(roll);
        let func = SCORE_OPT_FUNC.get(index)?;
        return func(self, &counts).ok();
    }

    pub fn score_by_option(&mut self, roll: &Roll, choice: usize) -> Option<i32> {
        let index = choice.checked_sub(1)?;
        return self.score_by_func_index(roll, index);
    }
}

/* Mutators */
impl ScoreCard {
    fn try_set(target: &mut Option<i32>, result: i32) -> Result<i32, i32> {
        return match target {
            Some(existing_value) => Err(*existing_value),
            None => {
                *target = Some(result);
                Ok(result)
            }
        };
    }

    fn score_aces(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = counts.die_value_total(1);
        return Self::try_set(&mut self.aces, result);
    }

    fn score_twos(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = counts.die_value_total(2);
        return Self::try_set(&mut self.twos, result);
    }

    fn score_threes(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = counts.die_value_total(3);
        return Self::try_set(&mut self.threes, result);
    }

    fn score_fours(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = counts.die_value_total(4);
        return Self::try_set(&mut self.fours, result);
    }

    fn score_fives(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = counts.die_value_total(5);
        return Self::try_set(&mut self.fives, result);
    }

    fn score_sixes(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = counts.die_value_total(6);
        return Self::try_set(&mut self.sixes, result);
    }

    fn score_three_of_a_kind(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = match counts.has_kind(3) {
            true => counts.total(),
            false => 0,
        };
        return Self::try_set(&mut self.three_of_a_kind, result);
    }

    fn score_four_of_a_kind(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = match counts.has_kind(4) {
            true => counts.total(),
            false => 0,
        };
        return Self::try_set(&mut self.four_of_a_kind, result);
    }

    fn score_full_house(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = match counts.has_exact(3) && counts.has_exact(2) {
            true => 25,
            false => 0,
        };
        return Self::try_set(&mut self.full_house, result);
    }

    fn score_small_straight(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = match counts.straight_len() >= 4 {
            true => 30,
            false => 0,
        };
        return Self::try_set(&mut self.small_straight, result);
    }

    fn score_large_straight(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = match counts.straight_len() == 5 {
            true => 40,
            false => 0,
        };
        return Self::try_set(&mut self.large_straight, result);
    }

    fn score_chance(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        let result = counts.total();
        return Self::try_set(&mut self.chance, result);
    }

    fn score_rustzee(&mut self, counts: &ValueCounts) -> Result<i32, i32> {
        if !counts.has_kind(5) {
            return Self::try_set(&mut self.rustzee, 0);
        }
        return match self.rustzee {
            Some(50) => {
                self.rustzee_bonus += 100;
                return Ok(self.rustzee_bonus);
            },
            _ => Self::try_set(&mut self.rustzee, 50),
        };
    }
}

