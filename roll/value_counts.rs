use crate::die::Die;

pub struct ValueCounts {
    counts: [i32; 6],
    total: i32,
}

impl ValueCounts {
    pub fn new(dice: &[Die]) -> ValueCounts {
        let mut counts = [0; 6];
        let mut total = 0;
        for die in dice {
            let index = (die.value() as usize) - 1;
            counts[index] += 1;
            total += die.value();
        }
        return ValueCounts {
            counts: counts,
            total: total,
        };
    }

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
        return self.total;
    }
}

