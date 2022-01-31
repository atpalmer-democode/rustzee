use crate::Roll;
use super::scorecard::ScoreCard;


pub type ScoreFunc = for<'sc, 'r> fn(&'sc mut ScoreCard, &'r Roll) -> Result<i32, i32>;

fn try_set(target: &mut Option<i32>, result: i32) -> Result<i32, i32> {
    return match target {
        Some(existing_value) => Err(*existing_value),
        None => {
            *target = Some(result);
            Ok(result)
        }
    };
}

pub fn score_aces(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = roll.die_value_total(1);
    return card.aces.try_set(result);
}

pub fn score_twos(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = roll.die_value_total(2);
    return card.twos.try_set(result);
}

pub fn score_threes(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = roll.die_value_total(3);
    return card.threes.try_set(result);
}

pub fn score_fours(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = roll.die_value_total(4);
    return card.fours.try_set(result);
}

pub fn score_fives(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = roll.die_value_total(5);
    return card.fives.try_set(result);
}

pub fn score_sixes(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = roll.die_value_total(6);
    return card.sixes.try_set(result);
}

pub fn score_three_of_a_kind(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = match roll.has_kind(3) {
        true => roll.total(),
        false => 0,
    };
    return try_set(&mut card.three_of_a_kind, result);
}

pub fn score_four_of_a_kind(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = match roll.has_kind(4) {
        true => roll.total(),
        false => 0,
    };
    return try_set(&mut card.four_of_a_kind, result);
}

pub fn score_full_house(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = match roll.has_exact(3) && roll.has_exact(2) {
        true => 25,
        false => 0,
    };
    return try_set(&mut card.full_house, result);
}

pub fn score_small_straight(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = match roll.straight_len() >= 4 {
        true => 30,
        false => 0,
    };
    return try_set(&mut card.small_straight, result);
}

pub fn score_large_straight(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = match roll.straight_len() == 5 {
        true => 40,
        false => 0,
    };
    return try_set(&mut card.large_straight, result);
}

pub fn score_chance(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    let result = roll.total();
    return try_set(&mut card.chance, result);
}

pub fn score_rustzee(card: &mut ScoreCard, roll: &Roll) -> Result<i32, i32> {
    if !roll.has_kind(5) {
        return try_set(&mut card.rustzee, 0);
    }
    return match card.rustzee {
        Some(50) => {
            card.rustzee_bonus += 100;
            return Ok(card.rustzee_bonus);
        },
        _ => try_set(&mut card.rustzee, 50),
    };
}

