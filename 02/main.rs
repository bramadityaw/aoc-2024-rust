use std::fs::File;
use std::io::{BufRead, BufReader};

const EXAMPLE_1_ANSWER : usize = 2;
const EXAMPLE_2_ANSWER : usize = 4;

const PART_1_ANSWER : usize = 663;

fn example() -> bool {
    part1("./example") == EXAMPLE_1_ANSWER &&
    part2("./example") == EXAMPLE_2_ANSWER
}

fn is_safe2(report: &str) -> bool {
    let mut prev : usize = 0;
    let mut consistency : u8 = 0;
    let mut duplicates : u8 = 0;

    for level in report.split(' ') {
        if prev == 0 {
            prev = level.parse::<usize>().unwrap();
            continue;
        }
        let curr = level.parse::<usize>().unwrap();
        let diff = prev.abs_diff(curr);
        if diff > 3 {
            return false
        }
        if prev == curr {
            if duplicates >= 1 {
                duplicates |= 1 << 1;
            }
            duplicates |= 1;
        }
        if prev < curr {
            consistency |= 1;
        }
        if prev > curr {
            consistency |= 1 << 1;
        }
        if consistency.count_ones() == 2 {
            consistency = 1;
            todo!();
        }
        println!("{prev_prev} {prev} {curr}");
        prev = curr;
    }

    consistency.count_ones() == 1 &&
    duplicates.count_ones() == 1
}

fn is_safe(report: &str) -> bool {
    let mut prev : usize = 0;
    let mut flag : u8= 0;

    for level in report.split(' ') {
        if prev == 0 {
            prev = level.parse::<usize>().unwrap();
            continue;
        }
        let curr = level.parse::<usize>().unwrap();
        let diff = prev.abs_diff(curr);
        if prev == curr || diff > 3 {
            return false
        }
        if prev < curr {
            flag |= 1;
        }
        if prev > curr {
            flag |= 1 << 1;
        }
        prev = curr;
    }

    flag.count_ones() == 1
}

fn part1(filename: &str) -> usize {
    let input = BufReader::new(File::open(filename).unwrap());
    let lines = input.lines();
    let mut safe_counter = 0;
    for line in lines {
        match line {
            Ok(l) => {
                if is_safe(&l) {
                    safe_counter += 1;
                }
            }
            Err(_) => break
        }
    }
    safe_counter
}

fn part2(filename: &str) -> usize {
    let input = BufReader::new(File::open(filename).unwrap());
    let lines = input.lines();
    let mut safe_counter = 0;

    for line in lines {
        match line {
            Ok(l) => {
                if is_safe2(&l) {
                    safe_counter += 1;
                }
            }
            Err(_) => break
        }
    }

    dbg!(safe_counter)
}

fn main() {
    if ! example() {
        panic!();
    }

    println!("part 1: {}", part1("./input"));
    assert_eq!(PART_1_ANSWER, part1("./input"));

    println!("part 2: {}", part2("./input"));
}
