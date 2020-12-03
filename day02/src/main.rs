use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct InputLine {
    rule_part_one: i64,
    rule_part_two: i64,
    character: String,
    password: String,
}

impl InputLine {
    fn new(line: &String) -> InputLine {
        let data: Vec<_> = line.split(|c| c == '-' || c == ' ' || c == ':').collect();

        InputLine {
            rule_part_one: data[0].parse::<i64>().unwrap(),
            rule_part_two: data[1].parse::<i64>().unwrap(),
            character: data[2].to_string(),
            password: data[4].to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("C:/git/aoc2020/day02/input")?;
    let reader = BufReader::new(file);

    let mut first_policy_valid_count = 0;
    let mut second_policy_valid_count = 0;

    for line in reader.lines() {
        if let Ok(l) = line {
            let entry = InputLine::new(&l);

            if valid_for_first_policy(&entry) {
                first_policy_valid_count += 1;
            }

            if valid_for_second_policy(&entry) {
                second_policy_valid_count += 1;
            }
        }
    };

    println!("First Policy Valid Count: {}", first_policy_valid_count);
    println!("Second Policy Valid Count: {}", second_policy_valid_count);
    
    Ok(())
}

fn valid_for_first_policy(entry: &InputLine) -> bool {
    let character_count = entry.password.matches(&entry.character).count();

    (character_count as i64) >= entry.rule_part_one && (character_count as i64) <= entry.rule_part_two
}

fn valid_for_second_policy(entry: &InputLine) -> bool {
    (entry.password.get((entry.rule_part_one as usize) - 1..(entry.rule_part_one as usize)) == Some(&entry.character)) 
        ^  (entry.password.get((entry.rule_part_two as usize) - 1..(entry.rule_part_two as usize)) == Some(&entry.character))
}