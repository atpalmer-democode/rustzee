mod roll;
use roll::Roll;


fn main() {
    let roll = Roll::roll();
    println!("Dice: {}", roll);
}
