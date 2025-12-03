use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    result::Result::Ok,
};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut id_str = String::new();
        reader.read_to_string(&mut id_str)?;
        fn invalid(id: usize) -> usize {
            let mut mult = 1;
            let mut a = 0;
            let mut b = id;
            while a != b && b != 0 {
                a += mult * (b % 10);
                mult *= 10;
                b /= 10;
            }
            if a == b && a >= mult / 10 {
                id
            } else {
                0
            }
        }
        Ok(id_str
            .split(',')
            .map(|s| s.split_once('-').unwrap_or(("", "")))
            .flat_map(|(start, end)| {
                start.parse::<usize>().expect("Start not int")
                    ..=end.parse::<usize>().expect("end not int")
            })
            .map(|id| invalid(id))
            .sum::<usize>())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut id_str = String::new();
        reader.read_to_string(&mut id_str)?;
        fn invalid(id: usize) -> usize {
            let total_len = id.ilog10() + 1;
            for chunk_size in 1..total_len {
                if total_len % chunk_size == 0 {
                    let num_chunks = total_len / chunk_size;
                    let chunk_mask = 10_usize.pow(chunk_size);
                    let first_chunk = id / 10_usize.pow(total_len - chunk_size);
                    let mut all_same = true;
                    for i in 0..num_chunks {
                        let shift = total_len - (i + 1) * chunk_size;
                        let chunk = (id / 10_usize.pow(shift)) % chunk_mask;
                        if chunk != first_chunk {
                            all_same = false;
                            break;
                        }
                    }
                    if all_same {
                        return id;
                    }
                }
            }
            0
        }
        Ok(id_str
            .split(',')
            .map(|s| s.split_once('-').unwrap_or(("", "")))
            .flat_map(|(start, end)| {
                start.parse::<usize>().expect("Start not int")
                    ..=end.parse::<usize>().expect("end not int")
            })
            .map(|id| invalid(id))
            .sum::<usize>())
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
