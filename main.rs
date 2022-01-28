mod die;
mod roll;
mod scoring;
mod turn;

use roll::Keep;
use turn::TurnState;
use scoring::scorecard::ScoreCard;

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

    pub fn get_usize(prompt: String) -> usize {
        loop {
            println!("{}", prompt);
            let val: String = text_io::read!();
            let result = val.parse::<usize>();
            match result {
                Err(_) => continue,
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
        println!("{}", &turn);
        println!("Rolls left: {}", turn.rolls_left());
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
        turn = turn.reroll(keep);
    }

    println!("Available ScoreCard options:");
    for line in scorecard.options(&turn.current()) {
        println!("{}", line);
    }

    let scoring_choice = console::get_usize(format!("Scoring choice:"));
    println!("Your choice: {}", scoring_choice);
    println!("Available? {}", scorecard.is_option_available(scoring_choice));

    let new_scorecard = scorecard.score_roll(&turn.current(), scoring_choice).unwrap();
    println!("New Score: {}", new_scorecard.total());

    scorecard.score_by_option(&turn.current(), scoring_choice)?;
    println!("Updated Score: {}", scorecard.total());

    return Ok(());
}
