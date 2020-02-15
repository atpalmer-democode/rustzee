mod die;
use die::Die;


fn main() {
    let die = Die::from(6);
    println!("Die: {}", die);
}
