use crate::Roll;
use super::scorecard::ScoreCard;


pub type ScoreFunc = for<'sc, 'r> fn(&'sc mut ScoreCard, &'r Roll) -> Result<i32, i32>;

pub fn score_aces(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.aces.try_score(roll);
}

pub fn score_twos(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.twos.try_score(roll);
}

pub fn score_threes(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.threes.try_score(roll);
}

pub fn score_fours(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.fours.try_score(roll);
}

pub fn score_fives(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.fives.try_score(roll);
}

pub fn score_sixes(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.sixes.try_score(roll);
}

pub fn score_three_of_a_kind(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.three_of_a_kind.try_score(roll);
}

pub fn score_four_of_a_kind(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.four_of_a_kind.try_score(roll);
}

pub fn score_full_house(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.full_house.try_score(roll);
}

pub fn score_small_straight(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.small_straight.try_score(roll);
}

pub fn score_large_straight(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.large_straight.try_score(roll);
}

pub fn score_chance(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    return card.chance.try_score(roll);
}

pub fn score_rustzee(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    if !roll.has_kind(5) {
        return card.rustzee.try_set(0);
    }
    return match card.rustzee.get() {
        Some(50) => {
            card.rustzee_bonus += 100;
            return Ok(card.rustzee_bonus);
        },
        _ => card.rustzee.try_set(50),
    };
}

