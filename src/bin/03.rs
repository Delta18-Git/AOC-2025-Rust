use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; 
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Initial solution I came up with:
        // fn solve(num: &[u8]) -> u16 {
        //     let len = num.len();
        //     let mut left_pos = 0;
        //     let mut mult_pos = 1;
        //     let mut val = 0;
        //     let mut curr;
        //     for _ in 1..len {
        //         curr = (num[len - 1] - b'0') as u16 + 10 * (num[len - 1 - mult_pos] - b'0') as u16;
        //         if val <= curr {
        //             val = curr;
        //             left_pos = mult_pos;
        //         }
        //         mult_pos += 1;
        //     }
        //     for mult_pos in 1..left_pos {
        //         curr = (val / 10) * 10 + (num[len - 1 - mult_pos] - b'0') as u16;
        //         if val <= curr {
        //             val = curr;
        //         }
        //     }
        //     val
        // }
        fn solve(num: &[u8]) -> usize {
            let len = num.len();

            let mut max_left = 0usize;
            let mut lpos = 0;

            for i in 0..(len - 1) {
                let digit = (num[i] - b'0') as usize;
                if digit > max_left {
                    max_left = digit;
                    lpos = i;
                }
            }

            let mut max_right = 0usize;

            for i in (lpos + 1)..len {
                let digit = (num[i] - b'0') as usize;
                if digit > max_right {
                    max_right = digit;
                }
            }
            max_left * 10 + max_right
        }
        Ok(reader
            .lines()
            .map(|s| s.expect("How did we get here!"))
            .map(|s| solve(s.as_bytes()))
            .sum())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        fn solve(num_bytes: &[u8]) -> usize {
            let len = num_bytes.len();
            let mut res = 0;
            let mut lpos = 0;
            for digits_left in (1..=12).rev() {
                let offset = len - lpos - digits_left + 1;
                let mut max = 0;
                let mut mpos = lpos;
                for i in lpos..(lpos + offset) {
                    let digit = (num_bytes[i] - b'0') as usize;
                    if digit > max {
                        max = digit;
                        mpos = i;
                    }
                }
                res = res * 10 + max;
                lpos = mpos + 1;
            }
            res
        }
        Ok(reader
            .lines()
            .map(|s| s.expect("How did we get here!"))
            .map(|s| solve(s.as_bytes()))
            .sum())
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
