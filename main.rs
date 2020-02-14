use std::fmt::{Display, Formatter, Result};

struct Die {
    value: i32,
}

impl Die {
    fn from(value: i32) -> Die {
        return Die { value: value };
    }
}

impl Display for Die {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{}", self.value);
    }
}

fn main() {
    let die = Die::from(6);
    println!("Die: {}", die);
}
