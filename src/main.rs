use dice_string_parser::possible_rolls;
use dice_string_parser::DiceDistrubution;

fn main() {
    // let dd = DiceDistrubution::new("5d10 + 50");
    // dd.ptable();
    let rolls = possible_rolls("2d4");
    println!("{:?}", rolls);
}
