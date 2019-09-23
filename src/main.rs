// use dice_string_parser::possible_rolls;
use dice_string_parser::DiceDistrubution;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let dd = DiceDistrubution::new("2d6 + 4");
    dd.ptable();
    println!("took {}", now.elapsed().as_millis());

    // let rolls = possible_rolls("2d4");
    // println!("{:?}", rolls);
}
