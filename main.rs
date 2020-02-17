mod die;
mod roll;
mod scoring;
mod turn;

use roll::Keep;
use turn::TurnState;

fn main() {
    let mut turn = TurnState::new();

    turn.roll();
    println!("{}", &turn);

    let keep = Keep::dice(true, true, true, false, false);
    println!("Keep: {}", keep);

    turn.reroll(keep);
    println!("{}", &turn);
}
