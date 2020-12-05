use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use colored::*;

fn main() -> io::Result<()> {
    let file = File::open("C:/git/aoc2020/day03/input")?;
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            let mut row: Vec<char> = Vec::new();

            for c in l.chars() {
                row.push(c);
            }

            map.push(row);
        }
    }

    println!("Map Height: {}", map.len());
    println!("Map Width: {}", map[0].len());

    let total_count = traverse(&map, (1, 1)) 
                        * traverse(&map, (1, 3))
                        * traverse(&map, (1, 5))
                        * traverse(&map, (1, 7))
                        * traverse(&map, (2, 1));

    println!("Final Answer: {}", total_count);

    Ok(())
}

fn traverse(map: &Vec<Vec<char>>, route: (i64, i64)) -> i64 {
    let mut tree_count = 0;
    let mut cur_pos: (i64, i64) = (0, 0);

    while cur_pos.0 < map.len() as i64 - 1 {
        cur_pos = update_position(&map, cur_pos, route);
        if map[cur_pos.0 as usize][cur_pos.1 as usize] == '#' {
            tree_count += 1;
        }
    }

    println!("Tree Count: {}", tree_count);
    tree_count
}

fn update_position(map: &Vec<Vec<char>>, pos: (i64, i64), route: (i64, i64)) -> (i64, i64) {
    let mut new_pos = (pos.0 + route.0, pos.1 + route.1);

    if new_pos.1 >= map[0].len() as i64 {
        new_pos.1 = new_pos.1 - map[0].len() as i64;
    }

    new_pos
}

fn print_map_line(map_line: &Vec<char>, cur_pos: i64) {
    let mut print_string = String::new();

    print_string.push_str(&map_line[..cur_pos as usize].iter().collect::<String>());
    print_string.push_str(&map_line[cur_pos as usize].to_string().blue());
    print_string.push_str(&map_line[cur_pos as usize + 1..].iter().collect::<String>());

    println!("{}", print_string)
}
