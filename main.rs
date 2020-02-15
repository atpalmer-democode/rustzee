mod roll;
use roll::Roll;


fn main() {
    let roll = Roll::of(6, 5, 4, 3, 2);
    println!("Dice: {}", roll);
}
