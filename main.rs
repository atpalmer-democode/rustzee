mod die;
mod roll;
mod scoring;
mod turn;

use roll::Keep;
use turn::TurnState;
use scoring::scorecard::ScoreCard;

mod console {
    use text_io;

    pub fn get_bool(prompt: &str) -> bool {
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

    pub fn get_usize(prompt: &str) -> usize {
        loop {
            println!("{}", prompt);
            let val: String = text_io::read!();
            let result = val.parse::<usize>();
            match result {
                Err(_) => eprintln!("Invalid entry: \"{}\"", val),
                Ok(x) => return x,
            }
        }
    }
}

fn main() -> Result<(), i32> {
    let mut scorecard = ScoreCard::new();
    println!("Score: {}", scorecard.total());

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
            let prompt = format!("Keep die: {}? [Y/N]", die);
            keepers[i] = console::get_bool(&prompt);
        }

        let keep = Keep::new(keepers);
        println!("Keeping: {}", keep);
        turn = turn.reroll(keep);
    }

    loop {
        println!("Dice: {}", turn.current());

        println!("Available ScoreCard options:");
        for line in scorecard.options(&turn.current()) {
            println!("{}", line);
        }

        let scoring_choice = console::get_usize("Scoring choice:");

        match scorecard.score_by_option(&turn.current(), scoring_choice) {
            Some(total) => {
                println!("Updated Score: {}", total);
                break;
            },
            None => eprintln!("Invalid option: \"{}\"\n", scoring_choice)
        }
    }

    return Ok(());
}
