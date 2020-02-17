mod die;
mod roll;
mod scoring;
mod turn;

use roll::Keep;
use turn::TurnState;

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
