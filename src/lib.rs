extern crate meval;
extern crate regex;
use rand::Rng;
use regex::Captures;
use regex::Regex;
use itertools::Itertools;

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

pub struct RollInfo {
    pub num: i64,
    pub size: i64,
}

impl RollInfo {
    pub fn roll_values(&self) -> Vec<i64> {
        //-> MultiProduct<std::ops::Range<i64>> {
        let mut ranges = Vec::new();
        for _ in 0..self.num {
            ranges.push(1..(self.size + 1));
        }

        let mut roll_values = Vec::new();
        for combination in ranges.into_iter().multi_cartesian_product() {
            roll_values.push(combination.iter().sum());
        }

        roll_values
    }
}

fn extract_dice_values(string: &str) -> (String, Vec<RollInfo>) {
    // TODO remove duplicate regex definition
    let mut roll_infos = Vec::new();

    let re = Regex::new(r"(?P<num_dice>[0-9]+)d(?P<size_dice>[0-9]+)").unwrap();
    let result = re
        .replace_all(string, |caps: &Captures| {
            roll_infos.push(RollInfo {
                num: caps["num_dice"].parse().unwrap(),
                size: caps["size_dice"].parse().unwrap(),
            });
            "{}"
        })
        .into_owned();

    (result, roll_infos)
}

pub fn dice_distribtion(string: &str) -> Vec<i64> {
    let (format_string, roll_infos) = extract_dice_values(string);
    let roll_numbers: Vec<i64> = roll_infos
        .iter()
        .map(|x| x.roll_values())
        .multi_cartesian_product()
        .map(|x| parce_dice_string(&format_dice_string(&format_string, x)))
        .collect();

    roll_numbers

    // let mut roll_distribution = HashMap::new();
    // for roll in roll_numbers {
    //     roll_distribution.entry(roll).and_modify(|e| *e += 1).or_insert(1);
    // }
    // println!("{:#?}", roll_distribution);
}

fn format_dice_string(dice_string: &str, rolls: Vec<i64>) -> String {
    let re = Regex::new(r"\{}").unwrap();
    let mut new_string = dice_string.to_string();

    for roll in rolls {
        new_string = re.replace(new_string.as_str(), roll.to_string().as_str()).to_string();
    }
    new_string
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
