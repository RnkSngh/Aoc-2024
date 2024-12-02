use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Enumerate;
use std::{string, u128};

fn main() -> io::Result<()> {
    let file = File::open("/Users/raunak/Code/Aoc-2024/01/src/input.txt")?;
    let reader = io::BufReader::new(file);
    let mut left: Vec<i128> = Vec::new();
    let mut right: Vec<i128> = Vec::new();

    for line in reader.lines() {
        let l = line?;
        let split: Vec<&str> = l.split_whitespace().collect();
        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    let mut sum: u128 = 0;

    left.sort();
    right.sort();

    for i in 0..left.len() {
        let distance: i128 = (right[i] - left[i]).try_into().unwrap();
        sum += distance.abs() as u128;
    }

    // Part 1
    println!("final val {}", sum);

    // Now we have to see how many times
    let mut similarity: i128 = 0;
    for i in 0..left.len() {
        let numOccurrences = right.iter().filter(|x| x == &&left[i]).count();
        similarity += left[i] * (numOccurrences as i128);
    }

    // Part 2
    println!("similarity {}", similarity);
    Ok(())
}
