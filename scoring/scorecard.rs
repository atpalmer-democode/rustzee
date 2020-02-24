use super::scoring;
use crate::roll::Roll;

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
        return items.iter().map(|x|{x.unwrap_or(0)}).sum();
    }

    pub fn total(&self) -> i32 {
        return self.top_total() + self.bottom_total();
    }

    pub fn options(&self, _roll: &Roll) -> Vec<String> {
        let items = [
            (self.aces, 1, "Aces"),
            (self.twos, 2, "Twos"),
            (self.threes, 3, "Threes"),
            (self.fours, 4, "Fours"),
            (self.fives, 5, "Fives"),
            (self.sixes, 6, "Sixes"),
            (self.three_of_a_kind, 7, "3 of a Kind"),
            (self.four_of_a_kind, 8, "4 of a Kind"),
            (self.full_house, 9, "Full House"),
            (self.small_straight, 10, "Sm. Straight"),
            (self.large_straight, 11, "Lg. Stright"),
            (self.rustzee, 12, "Rustzee"),
            (self.chance, 13, "Chance"),
        ];
        return items.iter()
            .filter(|x|{x.0.is_none()})
            .map(|x|{format!("{:>2}.) {}", x.1, x.2)}) // TODO: add back hypothetical scoring
            .collect();
    }

    fn score_func_by_option(choice: i32) -> Option<fn(&mut ScoreCard, &Roll) -> Result<i32, i32>> {
        return match choice {
            1 => Some(Self::score_aces),
            2 => Some(Self::score_twos),
            3 => Some(Self::score_threes),
            4 => Some(Self::score_fours),
            5 => Some(Self::score_fives),
            6 => Some(Self::score_sixes),
            7 => Some(Self::score_three_of_a_kind),
            8 => Some(Self::score_four_of_a_kind),
            9 => Some(Self::score_full_house),
            10 => Some(Self::score_small_straight),
            11 => Some(Self::score_large_straight),
            // TODO: rustzee
            13 => Some(Self::score_chance),
            _ => None,
        };
    }

    pub fn is_option_available(&self, option: i32) -> bool {
        return match Self::score_func_by_option(option) {
            Some(_) => true,
            None => false,
        };
    }

    pub fn score_roll(&self, roll: &Roll, option: i32) -> Option<ScoreCard> {
        let mut result = (*self).clone();
        let fopt = Self::score_func_by_option(option);
        if fopt.is_none() {
            return None;
        }
        let func = fopt.unwrap();
        let func_result = func(&mut result, roll);
        return match func_result {
            Ok(_) => Some(result),
            Err(_) => None,
        };
    }
}

/* Mutators */
impl ScoreCard {
    fn do_score(target: &mut Option<i32>, result: i32) -> Result<i32, i32> {
        if target.is_some() {
            return Err(target.unwrap());
        }
        *target = Some(result);
        return Ok(result);
    }

    pub fn score_by_option(&mut self, roll: &Roll, choice: i32) -> Result<i32, i32> {
        let fopt = Self::score_func_by_option(choice);
        return match fopt {
            Some(f) => f(self, roll),
            None => Err(0),
        };
    }

    fn score_aces(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.count_values(1) * 1;
        return Self::do_score(&mut self.aces, result);
    }

    fn score_twos(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.count_values(2) * 2;
        return Self::do_score(&mut self.aces, result);
    }

    fn score_threes(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.count_values(3) * 3;
        return Self::do_score(&mut self.aces, result);
    }

    fn score_fours(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.count_values(4) * 4;
        return Self::do_score(&mut self.aces, result);
    }

    fn score_fives(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.count_values(5) * 5;
        return Self::do_score(&mut self.aces, result);
    }

    fn score_sixes(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = roll.count_values(6) * 6;
        return Self::do_score(&mut self.aces, result);
    }

    fn score_three_of_a_kind(&mut self, roll: &Roll) -> Result<i32, i32> {
        let counts = scoring::ValueCounts::from(roll);
        let result = match counts.has_kind(3) {
            true => scoring::total(roll),
            false => 0,
        };
        return Self::do_score(&mut self.three_of_a_kind, result);
    }

    fn score_four_of_a_kind(&mut self, roll: &Roll) -> Result<i32, i32> {
        let counts = scoring::ValueCounts::from(roll);
        let result = match counts.has_kind(4) {
            true => scoring::total(roll),
            false => 0,
        };
        return Self::do_score(&mut self.four_of_a_kind, result);
    }

    fn score_full_house(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.full_house, scoring::full_house(roll));
    }

    fn score_small_straight(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.small_straight, scoring::small_straight(roll));
    }

    fn score_large_straight(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.large_straight, scoring::large_straight(roll));
    }

    fn score_chance(&mut self, roll: &Roll) -> Result<i32, i32> {
        let result = scoring::total(roll);
        return Self::do_score(&mut self.chance, result);
    }

    /* TODO: rustzee and rustzee_bonus */
}

