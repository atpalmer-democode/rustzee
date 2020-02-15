mod die;
use die::Die;
mod roll;
use roll::Roll;


fn main() {
    let roll = Roll::from(6, 5, 4, 3, 2);
    let die = Die::from(6);
    println!("Dice: {}", roll);
    println!("Die: {}", die);
}
