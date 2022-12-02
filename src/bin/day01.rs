// ****************************************************************************
// # Advent of Code – Day 1
//
// [Martin Schaer](www.schaerweb.com)
//
// ## Notes:
// - used BufRead and avoided unrequired traversing of vectors. If performance
//   is not an issue, see this solution from ThePrimeagen using split, map,
//   flat_map, sort_by, take, …: https://github.com/ThePrimeagen/aoc/blob/2022/src/bin/day1.rs
// - Error handling should be improved
// - Includes unit test
// - Should main return a specific type to be a better CLI tool?
// - Try gumdrop to parse CLI args
// ****************************************************************************

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn insert_in_order(top3: &mut [u32], cal: u32) {
    let mut i: usize = 0;
    let mut cal = cal;
    while i < 3 {
        let tmp = top3[i];
        if tmp <= cal {
            top3[i] = cal;
            cal = tmp
        }
        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut elves: Vec<u32> = vec![0];
    let mut elf = 0u16;
    let mut max_number = 0u32;
    let mut elf_with_max = 0u16;
    let mut top_3_total_calories = [0u32, 0u32, 0u32];

    // Read the whole file at once:
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // Using BufRead instead to support huge input files

    let file = File::open(file_path).expect("Unable to open file");
    let file = BufReader::new(file);

    for line in file.lines() {
        let line = line.expect("Unable to read line");
        if line.len() == 0 {
            insert_in_order(&mut top_3_total_calories, elves[elf as usize]);
            elf += 1;
            elves.push(0u32);
        } else {
            let e = &elves.get(elf as usize).expect("Unable to access elf");
            let x: u32 = line.parse().expect("Not a number");
            let total = *e + x;
            elves[elf as usize] = total;
            if total >= max_number {
                max_number = total;
                elf_with_max = elf.clone();
            }
        }
    }
    insert_in_order(&mut top_3_total_calories, elves[elf as usize]);

    println!(
        "Elf with max calories: {}, with {}",
        elf_with_max, max_number
    );
    let sum_top_3: u32 = top_3_total_calories.iter().sum();
    println!(
        "Top 3 total calories: {:?} sum: {}",
        top_3_total_calories, sum_top_3
    );
}

// ============================================================================
// Unit tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::insert_in_order;

    #[test]
    fn test1() {
        let mut arr = [0u32, 0u32, 0u32];
        insert_in_order(&mut arr, 2u32);
        insert_in_order(&mut arr, 3u32);
        insert_in_order(&mut arr, 1u32);
        assert_eq!(arr, [3u32, 2u32, 1u32])
    }

    #[test]
    fn test2() {
        let mut arr = [0u32, 0u32, 0u32];
        insert_in_order(&mut arr, 2u32);
        insert_in_order(&mut arr, 3u32);
        insert_in_order(&mut arr, 2u32);
        insert_in_order(&mut arr, 4u32);
        assert_eq!(arr, [4u32, 3u32, 2u32])
    }
}

