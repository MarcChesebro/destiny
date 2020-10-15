//! This is a crate for dice rolling utilities.
//! 
//! # Examples
//! Roll some dice using destiny::parse_dice_string:
//! ```
//! use destiny::parse_dice_string;
//!
//! println!("{}", parse_dice_string("1d4").unwrap());
//! println!("{}", parse_dice_string("1d6").unwrap());
//! println!("{}", parse_dice_string("2d6").unwrap());
//! println!("{}", parse_dice_string("1d8 + 3").unwrap());
//! println!("{}", parse_dice_string("1d6 + 2d8").unwrap());
//! ```
//! 
//! Calculate distributions using destiny::DiceDistribution:
//! ```
//! use destiny::DiceDistribution;
//! 
//! let dd = DiceDistribution::new("2d6");
//! dd.ptable();
//! /* this will output:
//! +------+--------+--------+---------+--------+
//! | Roll | #Rolls | Roll%  | Roll>=% | Roll<% |
//! +======+========+========+=========+========+
//! | 2    | 1      | 2.78%  | 100.00% | 0.00%  |
//! +------+--------+--------+---------+--------+
//! | 3    | 2      | 5.56%  | 97.22%  | 2.78%  |
//! +------+--------+--------+---------+--------+
//! | 4    | 3      | 8.33%  | 91.67%  | 8.33%  |
//! +------+--------+--------+---------+--------+
//! | 5    | 4      | 11.11% | 83.33%  | 16.67% |
//! +------+--------+--------+---------+--------+
//! | 6    | 5      | 13.89% | 72.22%  | 27.78% |
//! +------+--------+--------+---------+--------+
//! | 7    | 6      | 16.67% | 58.33%  | 41.67% |
//! +------+--------+--------+---------+--------+
//! | 8    | 5      | 13.89% | 41.67%  | 58.33% |
//! +------+--------+--------+---------+--------+
//! | 9    | 4      | 11.11% | 27.78%  | 72.22% |
//! +------+--------+--------+---------+--------+
//! | 10   | 3      | 8.33%  | 16.67%  | 83.33% |
//! +------+--------+--------+---------+--------+
//! | 11   | 2      | 5.56%  | 8.33%   | 91.67% |
//! +------+--------+--------+---------+--------+
//! | 12   | 1      | 2.78%  | 2.78%   | 97.22% |
//! +------+--------+--------+---------+--------+
//! */
//! ```

extern crate meval;
extern crate regex;
use prettytable::{Cell, Row, Table};

#[macro_use]
extern crate prettytable;

use itertools::Itertools;
use rand::Rng;
use regex::Captures;
use regex::Regex;
use std::collections::HashMap;

extern crate rayon;

use rayon::prelude::*;

/// Simulates the rolling of num_dice of size size_dice.
fn roll_dice(num_dice: usize, size_dice: usize) -> usize {
    let mut rng = rand::thread_rng();

    let mut total = 0;
    for _ in 0..num_dice {
        total += rng.gen_range(1, size_dice + 1);
    }

    total
}

/// Takes a string with dice notation and replaces them with a simulated roll.
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

/// Parses and evaluates Strings with dice notation.
///
/// # Examples
/// ```
/// use destiny::parse_dice_string;
///
/// let roll = parse_dice_string("1d6").unwrap();
/// assert!(roll >= 1 && roll <= 6);
/// ```
///
/// ```
/// use destiny::parse_dice_string;
///
/// let roll = parse_dice_string("1d6 + 3").unwrap();
/// assert!(roll >= 4 && roll <= 9);
/// ```
///
/// ```
/// use destiny::parse_dice_string;
///
/// let roll = parse_dice_string("2d6").unwrap();
/// assert!(roll >= 2 && roll <= 12);
/// ```
pub fn parse_dice_string(string: &str) -> Result<i64, meval::Error> {
    let result = meval::eval_str(replace_dice(string))?;
    Ok(result as i64)
}

struct RollInfo {
    pub num: i64,
    pub size: i64,
}

impl RollInfo {
    pub fn roll_values(&self) -> Vec<i64> {
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

    pub fn num_possible_rolls(&self) -> i64 {
        self.size.pow(self.num as u32)
    }
}

/// A struct used to hold the information about a dice distribution.
/// 
/// # Examples
/// ```
/// use destiny::DiceDistribution;
/// 
/// let dd = DiceDistribution::new("1d4");
/// 
/// assert_eq!(vec![1, 2, 3, 4], dd.possible_rolls);
/// ```
pub struct DiceDistribution {
    pub dice_string: String,
    pub possible_rolls: Vec<i64>,
    pub distribution: HashMap<i64, i64>,
    pub roll_percentages: HashMap<i64, f64>,
    pub roll_over: HashMap<i64, f64>,
    pub roll_under: HashMap<i64, f64>,

}

impl DiceDistribution {
    /// Creates a new DiceDistribution. This uses the supplied string to calculate the
    /// possible rolls, the distrribution of those rolls and the percentage chance to roll any particular value.
    pub fn new(dice_string: &str) -> DiceDistribution {
        let dice_string = String::from(dice_string);
        let possible_rolls = possible_rolls(&dice_string);
        let distribution = roll_distribution(&possible_rolls);
        let roll_percentages = roll_percentage(&distribution);
        let roll_over = roll_over_percentage(&roll_percentages);
        let roll_under = roll_under_percentage(&roll_percentages);

        DiceDistribution {
            dice_string,
            possible_rolls,
            distribution,
            roll_percentages,
            roll_over,
            roll_under
        }
    }

    /// Creates a prettytable::Table containing a representation of self.
    pub fn table(&self) -> Table {
        // distribution_table(&self.distribution, &self.roll_percentages)
        let mut tuples = Vec::new();

        for (roll, num_rolled) in &self.distribution {
            tuples.push([roll, num_rolled]);
        }
        tuples.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut table = Table::new();

        table.set_titles(row!["Roll", "#Rolls", "Roll%", "Roll>=%", "Roll<%"]);

        for tuple in tuples {
            let percent = format!("{:.2}%", self.roll_percentages[&tuple[0]] * 100f64);
            let over_percent = format!("{:.2}%", self.roll_over[&tuple[0]] * 100f64);
            let under_percent = format!("{:.2}%", self.roll_under[&tuple[0]] * 100f64);
            
            table.add_row(Row::new(vec![
                Cell::new(&tuple[0].to_string()),
                Cell::new(&tuple[1].to_string()),
                Cell::new(&percent),
                Cell::new(&over_percent),
                Cell::new(&under_percent)
            ]));
        }
        
        table
    }

    /// Creates and prints to stdout a table representation of self.
    pub fn ptable(&self) {
        &self.table().printstd();
    }
}

/// Calculates the complexity of a roll. This counts the total number of possible 
/// dice combinations. This is usefull if you want to make sure a dice roll is 
/// resonable to calculate before trying it. The best use case for this is if you
/// are taking user input. if a user tries to calculate the distribution of 
/// "8d10" the there are 100,000,000 possible dice rolls. Calculating a distribution
/// on this would take a very long time.
/// 
/// # Examples
/// ```
/// use destiny::roll_complexity;
/// 
/// let num_possibilities = roll_complexity("8d10");
/// assert_eq!(num_possibilities, 100_000_000)
/// ```
pub fn roll_complexity(dice_string: &str) -> i64 {

    let (_, roll_infos) = extract_dice_values(dice_string);

    roll_infos.iter()
    .map(|x| x.num_possible_rolls())
    .product()
}

/// Replaces all the dice notions in a string with a '{}' placeholder and creates a Vector of coresponding RollInfo structs
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

/// Calculates all the possible roll combinations for a given dice string.
///
/// # Examples
///
/// Calculate the possibilities of rolling 1d6:
/// ```
/// use destiny::possible_rolls;
///
/// let rolls = possible_rolls("1d6");
/// assert_eq!(vec![1, 2, 3, 4, 5, 6], rolls);
/// ```
///
/// Calulate the possible rolls of 1d4 + 2:
/// ```
/// use destiny::possible_rolls;
///
/// let rolls = possible_rolls("1d4 + 2");
/// assert_eq!(vec![3, 4, 5, 6], rolls);
/// ```
///
/// Calculating the possibilities of 2d4:
/// ```
/// use destiny::possible_rolls;
///
/// let rolls = possible_rolls("2d4");
/// assert_eq!(vec![2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7, 5, 6, 7, 8], rolls);
/// ```
pub fn possible_rolls(string: &str) -> Vec<i64> {
    let (format_string, roll_infos) = extract_dice_values(string);
    let roll_numbers: Vec<i64> = roll_infos
        .iter()
        .map(|x| x.roll_values())
        .multi_cartesian_product()
        .collect::<Vec<Vec<i64>>>()
        .par_iter()
        .map(|x| parse_dice_string(&format_dice_string(&format_string, x)))
        .filter_map(Result::ok)
        .collect();

    roll_numbers
}

/// Calculates how many times each roll could have rolled to show the distribution.
///
/// A hashmapp is returned with the key being the number rolled and the value being the amount it was rolled.
///
/// # Examples
/// ```
/// use destiny::{ possible_rolls, roll_distribution };
///
/// let rolls = possible_rolls("2d6");
/// let distribution = roll_distribution(&rolls);
///
/// // There is only one way to roll 2 on 2d6(Two 1's).
/// assert_eq!(1, distribution[&2]);
pub fn roll_distribution(roll_numbers: &Vec<i64>) -> HashMap<i64, i64> {
    let mut roll_distribution = HashMap::new();
    for roll in roll_numbers {
        roll_distribution
            .entry(*roll)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    roll_distribution
}

/// Takes a distribution and calculates the chance to roll every value
pub fn roll_percentage(distribution: &HashMap<i64, i64>) -> HashMap<i64, f64> {
    let total_rolls: i64 = distribution.values().sum();
    let mut roll_percentages = HashMap::new();

    for (roll, num_rolled) in distribution {
        roll_percentages.insert(*roll, *num_rolled as f64 / total_rolls as f64);
    }

    roll_percentages
}

fn roll_over_percentage(roll_percentages: &HashMap<i64, f64>) -> HashMap<i64, f64> {
    let mut roll_over = HashMap::new();
    for roll in roll_percentages.keys() {
        for (proll, percentage) in roll_percentages {
            if proll >= roll {
                roll_over.entry(*roll).and_modify(|e| *e += percentage).or_insert(*percentage);
            }
        }
    }
    
    roll_over
}

fn roll_under_percentage(roll_percentages: &HashMap<i64, f64>) -> HashMap<i64, f64> {
    let mut roll_under = HashMap::new();
    for roll in roll_percentages.keys() {
        roll_under.insert(*roll, 0f64);
        for (proll, percentage) in roll_percentages {
            if proll < roll {
                roll_under.entry(*roll).and_modify(|e| *e += percentage).or_insert(*percentage);
            }
        }
    }

    roll_under
}

fn format_dice_string(dice_string: &str, rolls: &Vec<i64>) -> String {
    let re = Regex::new(r"\{}").unwrap();
    let mut new_string = dice_string.to_string();

    for roll in rolls {
        new_string = re
            .replace(new_string.as_str(), roll.to_string().as_str())
            .to_string();
    }
    new_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dice_string() {
        for _ in 0..100 {
            let value = parse_dice_string("1d6").unwrap();
            assert!(value > 0 && value < 7);
        }
    }

    #[test]
    fn test_math() {
        let value = parse_dice_string("1d1+2").unwrap();
        assert_eq!(value, 3);
    }

    #[test]
    fn test_math_1() {
        let value = parse_dice_string("1d1 + 2").unwrap();
        assert_eq!(value, 3);
    }

    #[test]
    fn test_math_2() {
        let value = parse_dice_string("1d1 - 1d1 + 2").unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn test_math_3() {
        let value = parse_dice_string("3 + 1d1 * 2").unwrap();
        assert_eq!(value, 5);
    }

    #[test]
    fn test_math_4() {
        let value = parse_dice_string("(3 + 1d1) * 2").unwrap();
        assert_eq!(value, 8);
    }

    #[test]
    fn test_less_whitespace() {
        let value = parse_dice_string("(3+1d1)*2").unwrap();
        assert_eq!(value, 8);
    }

    #[test]
    fn test_more_whitespace() {
        let value = parse_dice_string("  (   3 + 1d1 ) *2 ").unwrap();
        assert_eq!(value, 8);
    }

    #[test]
    fn test_bad_string_1() -> Result<(), String> {
        let value = parse_dice_string("1d4 + asdf");

        match value {
            Ok(_) => Err(String::from("dice string '1d4 + asdf' should have returned an error but did not")),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_bad_string_2() -> Result<(), String> {
        let value = parse_dice_string("1da4 + 3");

        match value {
            Ok(_) => Err(String::from("dice string '1da4 + 3' should have returned an error but did not")),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_bad_string_3() -> Result<(), String> {
        let value = parse_dice_string("1d4 + 1d");

        match value {
            Ok(_) => Err(String::from("dice string '1d4 + 1d' should have returned an error but did not")),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_bad_string_4() -> Result<(), String> {
        let value = parse_dice_string("1d4 + d6");

        match value {
            Ok(_) => Err(String::from("dice string '1d4 + d6' should have returned an error but did not")),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_bad_string_6() -> Result<(), String> {
        let value = parse_dice_string("(1d4 + 1d6");

        match value {
            Ok(_) => Err(String::from("dice string '(1d4 + 1d6' should have returned an error but did not")),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn test_complexity_1() {
        assert_eq!(roll_complexity("2d6"), 36);
    }

    #[test]
    fn test_complexity_2() {
        assert_eq!(roll_complexity("2d6 + 2d6"), 1296);
    }

    #[test]
    fn test_complexity_3() {
        assert_eq!(roll_complexity("1d20 + 3d6"), 4320);
    }

    #[test]
    fn test_complexity_4() {
        assert_eq!(roll_complexity("8d10"), 100_000_000);
    }

    #[test]
    fn test_distribution_1() {
        let dd = DiceDistribution::new("1d4");
        assert_eq!(dd.possible_rolls, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_distribution_2() {
        let dd = DiceDistribution::new("2d4");
        assert_eq!(dd.possible_rolls, vec![2, 3, 4, 5, 3, 4, 5, 6, 4, 5, 6, 7, 5, 6, 7, 8]);
    }
}
