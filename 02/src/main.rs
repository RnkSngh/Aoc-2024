use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Enumerate;
use std::{string, u128};

fn isSafe(u: &Vec<i128>) -> bool {
    let flag: i8 = if u[1] < u[0] { -1 } else { 1 };
    // Checks that is safe:
        // All dffs are either decreasing or increasing
        // All have a difference between 1 and 3
    for (i, &value) in u.iter().enumerate() {
        if i == 0 {
            continue;
        }
        // -1 means decreasing, i.e. u[i]< u[i-1]
        if u[i - 1] == u[i] {
            return false;
        }

        if flag == -1 {
            // decreasing
            if u[i - 1] - u[i] > 3 || u[i - 1] <= u[i] {
                return false;
            }
        } else if flag == 1 {
            // increasing
            if u[i] - u[i - 1] > 3 || u[i] <= u[i - 1] {
                return false;
            }
        }
    }
    return true;
}

fn isSafeWithRemoval(u: &Vec<i128>) -> bool {
    for (i, &e) in u.iter().enumerate() {
        let rem = u
            .iter()
            .enumerate()
            .filter(|(idx, x)| idx != &i)
            .map(|(idx, x)| *x)
            .collect();
        if isSafe(&rem) {
            return true;
        }
    }
    return false;
}

fn main() -> io::Result<()> {
    let file =
        File::open("/Users/raunak/Code/Aoc-2024/02/src/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut count: u128 = 0;
    let mut countWithRemoval: u128 = 0;
    for line in reader.lines() {
        let l = line?;
        let split: Vec<&str> = l.split_whitespace().collect();

        let mut elems: Vec<i128> = Vec::new();
        for s in split {
            elems.push(s.parse().unwrap());
        }

        if isSafe(&elems) {
            count += 1;
        }
        if isSafeWithRemoval(&elems) {
            countWithRemoval += 1;
        }
    }
    // Part 1
    println!("safe count : {}", count);

    // Part 2
    println!("safe count with removal: {}", countWithRemoval);
    Ok(())
}
