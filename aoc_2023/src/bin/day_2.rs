use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

// Source: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part1(input: &Vec<String>) {
    let mut ids: Vec<i32> = vec![];
    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    'lines: for line in input {
        let segments: Vec<&str> = line.split(":").collect();

        let game = segments[0];
        let rounds: Vec<&str> = segments[1].split(';').collect();

        let re_game = Regex::new(r"\d+").unwrap();
        let re_rounds = Regex::new(r"\d+\s+(red|green|blue)").unwrap();

        let game_id: i32 = re_game.find(game).unwrap().as_str().parse::<i32>().unwrap();

        for round in rounds {
            let matches = re_rounds.find_iter(round);

            for m in matches {
                let array: Vec<&str> = m.as_str().split(" ").collect();

                let count: i32 = array[0].parse::<i32>().unwrap();
                let color: &str = array[1];

                if bag.get(color).unwrap() < &count {
                    continue 'lines;
                }
            }
        }

        ids.push(game_id);
    }

    println!("{:?}", ids.iter().sum::<i32>());
}

fn part2(input: &Vec<String>) {
    let mut sum: i32 = 0;

    for line in input {
        let segments: Vec<&str> = line.split(":").collect();

        let rounds: Vec<&str> = segments[1].split(';').collect();

        let re_rounds = Regex::new(r"\d+\s+(red|green|blue)").unwrap();

        let mut min_cubes = HashMap::new();

        for round in rounds {
            let matches = re_rounds.find_iter(round);

            for m in matches {
                let array: Vec<&str> = m.as_str().split(" ").collect();

                let count: i32 = array[0].parse::<i32>().unwrap();
                let color: &str = array[1];

                if !min_cubes.contains_key(color) {
                    min_cubes.insert(color, count);
                } else {
                    let curr = min_cubes.get(color).unwrap();

                    if curr < &count {
                        min_cubes.insert(color, count);
                    }
                }
            }
        }

        let power: i32 = min_cubes.values().copied().fold(1, |acc, x| acc * x);

        sum += power;
    }

    println!("{:?}", sum);
}

fn main() {
    let input = read_lines("./src/inputs/day_2.txt").unwrap();
    let lines: Vec<String> = input.into_iter().map(|m| m.unwrap()).collect();

    part1(&lines);
    part2(&lines);
}
