extern crate meval;
extern crate regex;
use prettytable::{Table, Row, Cell};

#[macro_use] extern crate prettytable;

use rand::Rng;
use regex::Captures;
use regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

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

struct RollInfo {
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

pub struct DiceDistrubution {
    dice_string: String,
    possible_rolls: Vec<i64>,
    distribution: HashMap<i64, i64>,
    roll_percentages: HashMap<i64, f64>,
    // roll_under: HashMap<i64, f64>,
    // roll_over: HashMap<i64, f64>
}

impl DiceDistrubution {

    pub fn new(dice_string: &str) -> DiceDistrubution {
        let dice_string = String::from(dice_string);
        let possible_rolls = possible_rolls(&dice_string);
        let distribution = roll_distribution(&possible_rolls);
        let roll_percentages = roll_percentage(&distribution);

        DiceDistrubution{
            dice_string,
            possible_rolls,
            distribution,
            roll_percentages
        }

    }

    pub fn ptable(&self) {
        distribution_table(&self.distribution, &self.roll_percentages);
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

pub fn possible_rolls(string: &str) -> Vec<i64> {
    let (format_string, roll_infos) = extract_dice_values(string);
    let roll_numbers: Vec<i64> = roll_infos
        .iter()
        .map(|x| x.roll_values())
        .multi_cartesian_product()
        .map(|x| parce_dice_string(&format_dice_string(&format_string, x)))
        .collect();

    roll_numbers
}

pub fn roll_distribution(roll_numbers: &Vec<i64>) -> HashMap<i64, i64> {
    let mut roll_distribution = HashMap::new();
    for roll in roll_numbers {
        roll_distribution.entry(*roll).and_modify(|e| *e += 1).or_insert(1);
    }

    roll_distribution
}

pub fn roll_percentage(distribution: &HashMap<i64, i64>) -> HashMap<i64, f64> {

    let total_rolls: i64 = distribution.values().sum();
    let mut roll_percentages = HashMap::new();

    for (roll, num_rolled) in distribution {
        roll_percentages.insert(*roll, *num_rolled as f64 / total_rolls as f64);
    }

    roll_percentages
}

pub fn distribution_table(distribution: &HashMap<i64, i64>, roll_percentages: &HashMap<i64, f64>) {//-> String {
    
    let mut tuples = Vec::new();

    for (roll, num_rolled) in distribution {
        tuples.push([roll, num_rolled]);
    }
    tuples.sort_by(|a, b| a[0].cmp(&b[0]));
    
    let mut table = Table::new();

    table.add_row(row!["Roll", "#Rolls", "Roll%"]);

    for tuple in tuples {

        let percent = format!("{:.2}%", roll_percentages[&tuple[0]] * 100f64);
        table.add_row(Row::new(vec![
            Cell::new(&tuple[0].to_string()),
            Cell::new(&tuple[1].to_string()),
            Cell::new(&percent)
        ]));
    }
    table.printstd();
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
