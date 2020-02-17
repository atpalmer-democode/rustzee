mod die;
mod roll;
mod scoring;
mod turn;

use roll::Keep;
use turn::TurnState;

use text_io;

fn main() {
    let mut turn = TurnState::new();

    turn.roll();
    println!("{}", &turn);

    while turn.has_rolls(3) {
        let mut keepers: [bool; 5] = [true; 5];
        for (i, die) in turn.die_iter().enumerate() {
            loop {
                println!("Keep die: {}? [Y/N]", die);
                let val: String = text_io::read!();
                let keep = match &*val {
                    "Y" | "y" => Some(true),
                    "N" | "n" => Some(false),
                    _ => None,
                };
                 match keep {
                    Some(x) => {
                        keepers[i] = x;
                        break;
                    },
                    None => {
                        println!("Try again");
                        continue;
                    },
                }
            }
        }

        let keep = Keep::new(keepers);
        println!("Keeping: {}", keep);
        turn.reroll(keep);
        println!("{}", &turn);
    }
}
