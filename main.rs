mod die;
mod roll;
mod scoring;
mod turn;

use roll::{Roll, Keep};
use turn::TurnState;
use scoring::scorecard::ScoreCard;

mod console {
    use text_io;
    use std::io::Write;

    pub fn get_bool(prompt: &str) -> bool {
        loop {
            print!("{} ", prompt);
            std::io::stdout().flush().unwrap();
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

    pub fn get_usize(prompt: &str) -> usize {
        loop {
            print!("{} ", prompt);
            std::io::stdout().flush().unwrap();
            let val: String = text_io::read!();
            let result = val.parse::<usize>();
            match result {
                Err(_) => eprintln!("Invalid entry: \"{}\"", val),
                Ok(x) => return x,
            }
        }
    }
}

fn play_turn() -> TurnState {
    let mut turn = TurnState::roll();

    while turn.has_rolls() {
        println!("Dice: {}", turn.current());
        println!("Rolls left: {}", turn.rolls_left());
        let roll_again = console::get_bool("Roll again? [Y/N]");
        if !roll_again {
            break;
        }
        let mut keepers: [bool; 5] = [true; 5];
        for (i, die) in turn.current().into_iter().enumerate() {
            let prompt = format!("Keep die #{}, showing: <{}>? [Y/N]", i + 1, die);
            keepers[i] = console::get_bool(&prompt);
        }

        let keep = Keep::new(keepers);
        println!("{}", keep);
        turn = turn.reroll(keep);
    }

    return turn;
}

fn score_turn(scorecard: &mut ScoreCard, roll: &Roll) -> usize {
    loop {
        println!("Dice: {}", roll);

        println!("Available ScoreCard options:");
        let options = scorecard.get_options(roll);
        for (opt, text, score, total) in &options {
            println!("{:>2}.) {:<14} points: {:3} [total: {:3}]", opt, text, score, total);
        }
        let opt_count = options.len();

        let scoring_choice = console::get_usize("Scoring choice:");

        if let None = scorecard.score_by_option(roll, scoring_choice) {
            eprintln!("Invalid option: \"{}\"\n", scoring_choice);
            continue;
        };

        println!("Updated Score: {}", scorecard.total());
        return opt_count - 1;
    }
}

fn main() {
    let mut scorecard = ScoreCard::new();
    println!("Score: {}", scorecard.total());

    loop {
        let turn = play_turn();
        let turns_left = score_turn(&mut scorecard, turn.current());
        if turns_left == 0 {
            break;
        }
    }
}
