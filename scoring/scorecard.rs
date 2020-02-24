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

    pub fn options(&self, roll: &Roll) -> Vec<String> {
        let items = [
            (self.aces, 1, "Aces", scoring::score_as(roll, 1)),
            (self.twos, 2, "Twos", scoring::score_as(roll, 2)),
            (self.threes, 3, "Threes", scoring::score_as(roll, 3)),
            (self.fours, 4, "Fours", scoring::score_as(roll, 4)),
            (self.fives, 5, "Fives", scoring::score_as(roll, 5)),
            (self.sixes, 6, "Sixes", scoring::score_as(roll, 6)),
            (self.three_of_a_kind, 7, "3 of a Kind", scoring::three_of_a_kind(roll)),
            (self.four_of_a_kind, 8, "4 of a Kind", scoring::four_of_a_kind(roll)),
            (self.full_house, 9, "Full House", scoring::full_house(roll)),
            (self.small_straight, 10, "Sm. Straight", scoring::small_straight(roll)),
            (self.large_straight, 11, "Lg. Stright", scoring::large_straight(roll)),
            (self.rustzee, 12, "Rustzee", scoring::rustzee(roll)),  // TODO: account for rustzee bonuses
            (self.chance, 13, "Chance", scoring::chance(roll)),
        ];
        return items.iter()
            .filter(|x|{x.0.is_none()})
            .map(|x|{format!("{:>2}.) {} (points: {})", x.1, x.2, x.3)})
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
        return Self::do_score(&mut self.aces, scoring::score_as(roll, 1));
    }

    fn score_twos(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.twos, scoring::score_as(roll, 2));
    }

    fn score_threes(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.threes, scoring::score_as(roll, 3));
    }

    fn score_fours(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.fours, scoring::score_as(roll, 4));
    }

    fn score_fives(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.fives, scoring::score_as(roll, 5));
    }

    fn score_sixes(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.sixes, scoring::score_as(roll, 6));
    }

    fn score_three_of_a_kind(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.three_of_a_kind, scoring::three_of_a_kind(roll));
    }

    fn score_four_of_a_kind(&mut self, roll: &Roll) -> Result<i32, i32> {
        return Self::do_score(&mut self.four_of_a_kind, scoring::four_of_a_kind(roll));
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
        return Self::do_score(&mut self.chance, scoring::chance(roll));
    }

    /* TODO: rustzee and rustzee_bonus */
}

