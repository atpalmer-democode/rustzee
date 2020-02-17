mod die;
mod roll;
mod scoring;
mod turn;

use roll::Keep;
use turn::TurnState;
use scorecard::ScoreCard;

mod scorecard {
    #[derive(Default)]
    pub struct ScoreCard {
        aces: i32,
        twos: i32,
        threes: i32,
        fours: i32,
        fives: i32,
        sixes: i32,
        three_of_a_kind: i32,
        four_of_a_kind: i32,
        full_house: i32,
        small_straight: i32,
        large_straight: i32,
        rustzee: i32,
        chance: i32,
        bonus: Vec<i32>,
    }

    impl ScoreCard {
        pub fn new() -> ScoreCard {
            return ScoreCard::default();
        }

        fn top_subtotal(&self) -> i32 {
            let items = [
                self.aces,
                self.twos,
                self.threes,
                self.fours,
                self.fives,
                self.sixes,
            ];
            return items.iter().sum();
        }

        fn top_bonus(&self) -> i32 {
            return match self.top_subtotal() >= 63 {
                true => 35,
                false => 0,
            };
        }

        fn top_total(&self) -> i32 {
            return self.top_subtotal() + self.top_bonus();
        }

        fn rustzee_bonus(&self) -> i32 {
            return self.bonus.iter().sum();
        }

        fn bottom_total(&self) -> i32 {
            let items = [
                self.three_of_a_kind,
                self.four_of_a_kind,
                self.full_house,
                self.small_straight,
                self.large_straight,
                self.rustzee,
                self.chance,
                self.rustzee_bonus(),
            ];
            return items.iter().sum();
        }

        pub fn total(&self) -> i32 {
            return self.top_total() + self.bottom_total();
        }
    }
}

mod console {
    use text_io;

    pub fn get_bool(prompt: String) -> bool {
        loop {
            println!("{}", prompt);
            let val: String = text_io::read!();
            let result = match &*val {
                "Y" | "y" => Some(true),
                "N" | "n" => Some(false),
                _ => None,
            };
            match result {
                None => continue,
                Some(x) => return x,
            }
        }
    }
}

fn main() {
    let scorecard = ScoreCard::new();
    println!("Score: {}", scorecard.total());

    let mut turn = TurnState::new();

    turn.roll();

    while turn.has_rolls(3) {
        println!("{}", &turn);
        let roll_again = console::get_bool(format!("Roll again? [Y/N]"));
        if !roll_again {
            break;
        }
        let mut keepers: [bool; 5] = [true; 5];
        for (i, die) in turn.die_iter().enumerate() {
            let prompt = format!("Keep die: {}? [Y/N]", die);
            keepers[i] = console::get_bool(prompt);
        }

        let keep = Keep::new(keepers);
        println!("Keeping: {}", keep);
        turn.reroll(keep);
    }
}
