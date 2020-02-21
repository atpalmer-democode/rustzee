mod die;
mod roll;
mod scorecard;
mod scoring;
mod turn;

use roll::Keep;
use turn::TurnState;
use scorecard::ScoreCard;

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
    let mut scorecard = ScoreCard::new();
    println!("Score: {}", scorecard.total());

    let mut turn = TurnState::new();

    turn.roll();

    while turn.has_rolls() {
        println!("{}", &turn);
        println!("Rolls left: {} of {}", turn.rolls_left(), TurnState::MAX_ROLLS);
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

    println!("Available ScoreCard options:");
    for line in scorecard.options(&turn.current()) {
        println!("{}", line);
    }

    scorecard.score_aces(&turn.current());
    scorecard.score_twos(&turn.current());
    scorecard.score_threes(&turn.current());
    scorecard.score_fours(&turn.current());
    scorecard.score_fives(&turn.current());
    scorecard.score_sixes(&turn.current());
    scorecard.score_three_of_a_kind(&turn.current());
    scorecard.score_four_of_a_kind(&turn.current());
    scorecard.score_small_straight(&turn.current());
    scorecard.score_large_straight(&turn.current());
    scorecard.score_full_house(&turn.current());
    scorecard.score_chance(&turn.current());
    println!("Score: {}", scorecard.total());
}
