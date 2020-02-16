mod die;
mod roll;
use roll::{Roll, Keep};
mod scoring;


fn main() {
    let roll = Roll::roll();
    println!("Dice: {}", roll);

    let keep = Keep::dice(true, true, true, false, false);
    println!("Keep: {}", keep);

    let reroll = roll.reroll(keep);
    println!("Reroll: {}", reroll);
    println!("Score: {}", scoring::total(&reroll));
    println!("Score as 6s: {}", scoring::score_as(&reroll, 6));
    println!("Score as 5s: {}", scoring::score_as(&reroll, 5));
    println!("Score as 4s: {}", scoring::score_as(&reroll, 4));
    println!("Score as 3s: {}", scoring::score_as(&reroll, 3));
    println!("Score as 2s: {}", scoring::score_as(&reroll, 2));
    println!("Score as 1s: {}", scoring::score_as(&reroll, 1));
    println!("4 of a kind: {}", scoring::four_of_a_kind(&reroll));
    println!("3 of a kind: {}", scoring::three_of_a_kind(&reroll));
    println!("Rustzee: {}", scoring::rustzee(&reroll));
}
