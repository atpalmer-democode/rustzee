use crate::roll::Roll;

pub fn total(roll: &Roll) -> i32 {
    let mut result: i32 = 0;
    for die in roll.into_iter() {
        result += die.value();
    }
    return result;
}

pub fn score_as(roll: &Roll, value: i32) -> i32 {
    let count = roll.count_values(value);
    return count * value;
}
