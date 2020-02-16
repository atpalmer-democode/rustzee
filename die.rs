use std::fmt::{Debug, Display, Formatter, Result};
extern crate rand;
use rand::Rng;

pub struct Die {
    value: i32,
}

impl Die {
    pub fn roll() -> Die {
        let mut rng = rand::thread_rng();
        return Die {
            value: rng.gen_range(1, 7),
        };
    }
}

impl From<i32> for Die {
    fn from(value: i32) -> Die {
        assert!(value >=1 && value <= 6, "Die value must be 1-6");
        return Die { value: value };
    }
}

impl From<&Die> for Die {
    fn from(value: &Die) -> Die {
        return Die::from(value.value);
    }
}

impl Display for Die {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{}", self.value);
    }
}

impl Debug for Die {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return Display::fmt(self, f);
    }
}
