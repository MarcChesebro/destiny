use dice_string_parser::DiceDistrubution;

fn main() {
    // let roll_numbers = possible_rolls("2d6");
    // let distribution = roll_distribution(roll_numbers);
    // let roll_percentages = roll_percentage(&distribution);
    // distribution_table(distribution, roll_percentages);

    let dd = DiceDistrubution::new("5d10 + 50");
    dd.ptable();
}