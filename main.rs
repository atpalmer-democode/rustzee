mod roll;
use roll::Roll;


fn main() {
    let roll = Roll::from(6, 5, 4, 3, 2);
    println!("Dice: {}", roll);
}
