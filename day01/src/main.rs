use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let file = File::open("C:/git/aoc2020/day01/input")?;
    let reader = BufReader::new(file);

    let mut set: HashSet<i64> = HashSet::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let l = l.parse::<i64>().unwrap();
            set.insert(l);
        }
    }

    for val in set.iter() {
        match find_two_sum(&set, 2020 - *val) {
            Some(two_sol) => {
                println!("Two Sum Solution: {} * {} = {}", two_sol, *val, two_sol * *val);
            },
            _ => {},
        }
    }

    for val in set.iter() {
        match find_three_sum(&set, *val) {
            Some(three_sol) => {
                println!("Three Sum Solution: {} * {} * {} = {}", three_sol.0, three_sol.1, three_sol.2, three_sol.0 * three_sol.1 * three_sol.2);
            },
            _ => {},
        }
    }

    Ok(())
}

fn find_two_sum(set: &HashSet<i64>, target: i64) -> Option<i64> {
    match set.get(&target) {
        Some(val) => {
            return Some(*val);
        },
        None => {
            return None;
        }
    }
}

fn find_three_sum(set: &HashSet<i64>, target: i64) -> Option<(i64, i64, i64)> {
    for val in set {
        match find_two_sum(set, 2020 - val - target) {
            Some(ret) => return Some((*val, target, ret)),
            _ => {},
        }
    };
    None
}