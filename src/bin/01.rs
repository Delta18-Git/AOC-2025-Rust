use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result::Ok;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let dial_directives = reader.lines();
        let mut dial_value = 50;
        let mut zero_count = 0;
        for directive in dial_directives {
            let directive_safe = match directive {
                Ok(b) => b,
                Err(err) => panic!("{err:?}"),
            };
            let (direction, offset) = directive_safe.split_at(1);
            let mut offset_int = match offset.parse::<isize>() {
                Ok(num) => num,
                Err(err) => panic!("Error parsing int: {err:?}"),
            };
            offset_int = match direction {
                "L" => -offset_int,
                "R" => offset_int,
                _ => panic!("Invalid direction: {direction}"),
            };
            dial_value = (dial_value + offset_int) % 100;
            if dial_value == 0 {
                zero_count += 1;
            }
        }
        Ok(zero_count)
    }

    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let dial_directives = reader.lines();
        let mut dial_value = 50;
        let mut zero_count = 0;
        for directive in dial_directives {
            let directive_safe = match directive {
                Ok(b) => b,
                Err(err) => panic!("{err:?}"),
            };
            let (direction, offset) = directive_safe.split_at(1);
            let offset_int = match offset.parse::<isize>() {
                Ok(num) => num,
                Err(err) => panic!("Error parsing int: {err:?}"),
            };
            let turns = offset_int / 100;
            zero_count += turns.abs_diff(0);
            let rotation = offset_int % 100;
            match direction {
                "R" => {
                    if dial_value + rotation >= 100 {
                        zero_count += 1;
                    }
                    dial_value = (dial_value + rotation).rem_euclid(100);
                }
                "L" => {
                    if dial_value > 0 && dial_value - rotation <= 0 {
                        zero_count += 1;
                    }
                    dial_value = (dial_value - rotation).rem_euclid(100);
                }
                _ => panic!("Invalid direction: {direction}"),
            };
        }
        Ok(zero_count)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
