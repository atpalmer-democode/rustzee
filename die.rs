use std::fmt::{Display, Formatter, Result};

pub struct Die {
    value: i32,
}

impl Die {
    pub fn from(value: i32) -> Die {
        assert!(value >=1 && value <= 6, "Die value must be 1-6");
        return Die { value: value };
    }
}

impl Display for Die {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{}", self.value);
    }
}
