use std::fmt::{Debug, Display, Formatter, Result};

pub struct Die {
    value: i32,
}

impl From<i32> for Die {
    fn from(value: i32) -> Die {
        assert!(value >=1 && value <= 6, "Die value must be 1-6");
        return Die { value: value };
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
