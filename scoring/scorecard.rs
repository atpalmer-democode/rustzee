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
pub struct ScoreCard {
    pub(crate) aces: Option<i32>,
    pub(crate) twos: Option<i32>,
    pub(crate) threes: Option<i32>,
    pub(crate) fours: Option<i32>,
    pub(crate) fives: Option<i32>,
    pub(crate) sixes: Option<i32>,
    pub(crate) three_of_a_kind: Option<i32>,
    pub(crate) four_of_a_kind: Option<i32>,
    pub(crate) full_house: Option<i32>,
    pub(crate) small_straight: Option<i32>,
    pub(crate) large_straight: Option<i32>,
    pub(crate) rustzee: Option<i32>,
    pub(crate) chance: Option<i32>,
    pub(crate) rustzee_bonus: i32,
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
        let func = SCORE_OPT_FUNC.get(index)?;
        return func(self, &roll).ok();
    }

    pub fn score_by_option(&mut self, roll: &Roll, choice: usize) -> Option<i32> {
        let index = choice.checked_sub(1)?;
        return self.score_by_func_index(roll, index);
    }
}

