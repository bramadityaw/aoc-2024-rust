use std::fs::File;
use std::io::{BufRead, BufReader};

// Part 1
use std::iter::zip;

// Part 2
use std::collections::HashMap;

const EXAMPLE_1_ANSWER : usize = 11;
const EXAMPLE_2_ANSWER : usize = 31;

fn parse_pair(line: &str) -> (usize, usize) {
    let pairs : Vec<usize> = line.split_whitespace()
        .map(|l| l.parse::<usize>().unwrap() )
        .collect();
    assert_eq!(2, pairs.len());

    (pairs[0], pairs[1])
}

fn example() -> bool {
    part1("./1.example") == EXAMPLE_1_ANSWER &&
    part2("./1.example") == EXAMPLE_2_ANSWER
}

fn part1(filename: &str) -> usize {
    let input = BufReader::new(File::open(filename).unwrap());
    let lines = input.lines();
    let mut list1 : Vec<usize> = vec![];
    let mut list2 : Vec<usize> = vec![];
    for line in lines {
        match line {
            Ok(l) => {
                let (first, scnd) = parse_pair(&l);
                list1.push(first);
                list2.push(scnd);
            }
            Err(_) => break
        }
    }
    list1.sort(); list2.sort();
    let total_distance = zip(list1, list2)
                            .map(|e| e.0.abs_diff(e.1))
                            .reduce(|acc, e| acc + e)
                            .unwrap();

    total_distance
}

fn part2(filename: &str) -> usize {
    let input = BufReader::new(File::open(filename).unwrap());
    let lines = input.lines();

    let mut list1 : Vec<usize> = vec![];
    let mut list2 : Vec<usize> = vec![];

    for line in lines {
        match line {
            Ok(l) => {
                let (first, scnd) = parse_pair(&l);
                list1.push(first);
                list2.push(scnd);
            }
            Err(_) => break
        }
    }

    let mut map = HashMap::new();

    for num in list1.clone().into_iter() {
        map.insert(num, 0);
    }

    for (key, val) in map.iter_mut() {
        for num in list2.clone().into_iter() {
            if *key == num {
                *val += 1;
            }
        }
    }

      let sim_score = list1.clone().into_iter()
         .map(|e| e * map.get(&e).unwrap())
         .reduce(|acc, e| acc + e)
         .unwrap();

    dbg!(sim_score)
}

fn main() {
    if ! example() {
        panic!();
    }

    println!("part 1: {}", part1("./1.input"));
    println!("part 2: {}", part2("./1.input"));
}
