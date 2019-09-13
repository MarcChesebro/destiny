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

    #[test]
    fn test_math() {

        let value = parce_dice_string("1d1+2");
        assert_eq!(value, 3);
    }

    #[test]
    fn test_math_1() {

        let value = parce_dice_string("1d1 + 2");
        assert_eq!(value, 3);
    }

    #[test]
    fn test_math_2() {

        let value = parce_dice_string("1d1 - 1d1 + 2");
        assert_eq!(value, 2);
    }

    #[test]
    fn test_math_3() {

        let value = parce_dice_string("3 + 1d1 * 2");
        assert_eq!(value, 5);
    }

    #[test]
    fn test_math_4() {

        let value = parce_dice_string("(3 + 1d1) * 2");
        assert_eq!(value, 8);
    }
}
