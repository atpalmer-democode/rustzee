use crate::scoring;
use crate::roll::Roll;

#[derive(Default)]
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
    bonus: Vec<i32>,
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
        return items.iter().map(|x|{x.unwrap_or(0)}).sum();
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

    fn rustzee_bonus(&self) -> i32 {
        return self.bonus.iter().sum();
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
            Some(self.rustzee_bonus()),
        ];
        return items.iter().map(|x|{x.unwrap_or(0)}).sum();
    }

    pub fn total(&self) -> i32 {
        return self.top_total() + self.bottom_total();
    }
}

/* Mutators */
impl ScoreCard {
    pub fn score_aces(&mut self, roll: &Roll) {
        self.aces = Some(scoring::score_as(roll, 1));
    }

    pub fn score_twos(&mut self, roll: &Roll) {
        self.twos = Some(scoring::score_as(roll, 2));
    }

    pub fn score_threes(&mut self, roll: &Roll) {
        self.threes = Some(scoring::score_as(roll, 3));
    }

    pub fn score_fours(&mut self, roll: &Roll) {
        self.fives = Some(scoring::score_as(roll, 4));
    }

    pub fn score_fives(&mut self, roll: &Roll) {
        self.fives = Some(scoring::score_as(roll, 5));
    }

    pub fn score_sixes(&mut self, roll: &Roll) {
        self.sixes = Some(scoring::score_as(roll, 6));
    }

    pub fn score_three_of_a_kind(&mut self, roll: &Roll) {
        self.three_of_a_kind = Some(scoring::three_of_a_kind(roll));
    }

    pub fn score_four_of_a_kind(&mut self, roll: &Roll) {
        self.four_of_a_kind = Some(scoring::four_of_a_kind(roll));
    }

    pub fn score_full_house(&mut self, roll: &Roll) {
        self.full_house = Some(scoring::full_house(roll));
    }

    pub fn score_small_straight(&mut self, roll: &Roll) {
        self.small_straight = Some(scoring::small_straight(roll));
    }

    pub fn score_large_straight(&mut self, roll: &Roll) {
        self.large_straight = Some(scoring::large_straight(roll));
    }

    pub fn score_chance(&mut self, roll: &Roll) {
        self.chance = Some(scoring::chance(roll));
    }

    /* TODO: rustzee and rustzee_bonus */
}

