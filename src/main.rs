use dice_string_parser::parce_dice_string;

fn main() {
    let mut values = Vec::new();

    for _ in 0..7000 {
        values.push(parce_dice_string("1 + 2d6 + 5 + 1d8"))
    }

    println!("{}", values.iter().sum::<i64>() / values.len() as i64);

    for _ in 0..10 {
        println!("{}", parce_dice_string("1 + 2d6 + 5 + 1d8"));
    }
}
