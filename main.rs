mod die;
mod roll;
mod scoring;

use std::fmt;
use roll::{Roll, Keep};

struct TurnState {
    current: Option<Roll>,
    roll_count: i32,
}

impl TurnState {
    fn new() -> TurnState {
        return TurnState {
            current: None,
            roll_count: 0,
        };
    }

    fn roll(&mut self) {
        self.current = Some(Roll::roll());
        self.roll_count = self.roll_count + 1;
    }
}

impl fmt::Display for TurnState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self.current {
            Some(x) => write!(f, "{}", x),
            None => write!(f, ""),
        };
    }
}

fn display(roll: &Roll) {
    println!("Roll: {}", roll);
    println!("Score as 1s: {}", scoring::score_as(&roll, 1));
    println!("Score as 2s: {}", scoring::score_as(&roll, 2));
    println!("Score as 3s: {}", scoring::score_as(&roll, 3));
    println!("Score as 4s: {}", scoring::score_as(&roll, 4));
    println!("Score as 5s: {}", scoring::score_as(&roll, 5));
    println!("Score as 6s: {}", scoring::score_as(&roll, 6));
    println!("3 of a kind: {}", scoring::three_of_a_kind(&roll));
    println!("4 of a kind: {}", scoring::four_of_a_kind(&roll));
    println!("Full House: {}", scoring::full_house(&roll));
    println!("Small Straight: {}", scoring::small_straight(&roll));
    println!("Large Straight: {}", scoring::large_straight(&roll));
    println!("Rustzee: {}", scoring::rustzee(&roll));
    println!("Chance: {}", scoring::chance(&roll));
}

fn main() {
    let mut turn = TurnState::new();

    turn.roll();

    println!("Turn: {}", &turn);

    match turn.current {
        Some(x) => display(&x),
        None => {},
    };

    let roll = Roll::roll();
    display(&roll);

    let keep = Keep::dice(true, true, true, false, false);
    println!("Keep: {}", keep);

    let reroll = roll.reroll(keep);
    display(&reroll);
}
