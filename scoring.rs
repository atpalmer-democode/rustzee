use crate::roll::Roll;

pub fn total(roll: &Roll) -> i32 {
    let mut result: i32 = 0;
    for die in roll.into_iter() {
        result += die.value();
    }
    return result;
}
