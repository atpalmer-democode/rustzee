mod roll;
use roll::{Roll, Keep};


fn main() {
    let roll = Roll::roll();
    println!("Dice: {}", roll);

    let keep = Keep::dice(true, true, true, false, false);
    println!("Keep: {}", keep);

    let reroll = roll.reroll(keep);
    println!("Reroll: {}", reroll);
}
