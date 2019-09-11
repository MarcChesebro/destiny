extern crate meval;
extern crate regex;
use rand::Rng;
use regex::Captures;
use regex::Regex;

fn roll_dice(num_dice: usize, size_dice: usize) -> usize {
    let mut rng = rand::thread_rng();

    let mut total = 0;
    for _ in 0..num_dice {
        total += rng.gen_range(1, size_dice + 1);
    }

    total
}

fn replace_dice(string: &str) -> String {
    let re = Regex::new(r"(?P<num_dice>[0-9]+)d(?P<size_dice>[0-9]+)").unwrap();
    let result = re.replace_all(string, |caps: &Captures| {
        format!(
            "{}",
            roll_dice(
                caps["num_dice"].parse().unwrap(),
                caps["size_dice"].parse().unwrap()
            )
        )
    });
    format!("{}", result)
}

pub fn parce_dice_string(string: &str) -> i64 {
    let result = meval::eval_str(replace_dice(string)).unwrap();
    result.trunc() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parce_dice_string() {
        
        for _ in 0..100 {
            let value = parce_dice_string("1d6");
            assert!(value > 0 && value < 7);
        }
    }
}
