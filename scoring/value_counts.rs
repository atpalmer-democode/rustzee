use crate::roll::Roll;
use crate::die::Die;
use super::helpers;

pub struct ValueCounts<'roll> {
    counts: [i32; 6],
    roll: &'roll Roll,
}

impl<'r> ValueCounts<'r> {
    fn count(&self, die: &Die) -> i32 {
        let die_value = die.value() as usize;
        let index = die_value - 1;
        return self.counts[index];
    }

    fn count_value(&self, value: i32) -> i32 {
        let die = Die::from(value);
        return self.count(&die);
    }

    pub fn die_value_total(&self, value: i32) -> i32 {
        let count = self.count_value(value);
        return count * value;
    }

    pub fn has_exact(&self, count: i32) -> bool {
        for value in (1..7).rev() {
            let die = Die::from(value);
            if self.count(&die) == count {
                return true;
            };
        }
        return false;
    }

    pub fn has_kind(&self, kind: i32) -> bool {
        for value in (1..7).rev() {
            let die = Die::from(value);
            if self.count(&die) >= kind {
                return true;
            };
        }
        return false;
    }

    pub fn straight_len(&self) -> i32 {
        let mut maxlen = 0;
        let mut len = 0;
        for value in 1..7 {
            let die = Die::from(value);
            len = match self.count(&die) > 0 {
                true => len + 1,
                false => 0,
            };
            maxlen = match len > maxlen {
                true => len,
                false => maxlen,
            };
        }
        return maxlen;
    }

    pub fn total(&self) -> i32 {
        return helpers::total(self.roll);
    }
}

impl<'roll> From<&'roll Roll> for ValueCounts<'roll> {
    fn from(roll: &'roll Roll) -> ValueCounts<'roll> {
        let mut counts = [0; 6];
        for die in roll {
            let die_value = die.value() as usize;
            let index = die_value - 1;
            counts[index] += 1;
        }
        return ValueCounts {
            counts: counts,
            roll: roll,
        };
    }
}

