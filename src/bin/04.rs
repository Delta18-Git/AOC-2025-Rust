use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let width = content.lines().next().map(|l| l.len()).unwrap_or(0);
        let mut grid =
            Vec::with_capacity(content.len() + 2 * (width + 2) + content.lines().count() * 2);
        let mut height: usize = 0;
        grid.extend([b'.'].repeat(width + 2));
        for line in content.lines() {
            height += 1;
            grid.push(b'.');
            grid.extend(line.bytes());
            grid.push(b'.');
        }
        grid.extend([b'.'].repeat(width + 2));
        let mut result: usize = 0;
        for x in 1..=width {
            for y in 1..=height {
                if grid[y * (width + 2) + x] != b'@' {
                    continue;
                }
                let mut neigh: usize = 0;
                neigh += if grid[(y - 1) * (width + 2) + (x - 1)] == b'@' {
                    1
                } else {
                    0
                };
                neigh += if grid[(y - 1) * (width + 2) + (x + 0)] == b'@' {
                    1
                } else {
                    0
                };
                neigh += if grid[(y + 1) * (width + 2) + (x - 1)] == b'@' {
                    1
                } else {
                    0
                };
                neigh += if grid[(y + 0) * (width + 2) + (x - 1)] == b'@' {
                    1
                } else {
                    0
                };
                neigh += if grid[(y + 1) * (width + 2) + (x + 0)] == b'@' {
                    1
                } else {
                    0
                };
                neigh += if grid[(y + 0) * (width + 2) + (x + 1)] == b'@' {
                    1
                } else {
                    0
                };
                neigh += if grid[(y + 1) * (width + 2) + (x + 1)] == b'@' {
                    1
                } else {
                    0
                };
                neigh += if grid[(y - 1) * (width + 2) + (x + 1)] == b'@' {
                    1
                } else {
                    0
                };
                if neigh < 4 {
                    result += 1
                }
            }
        }
        Ok(result)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let width = content.lines().next().map(|l| l.len()).unwrap_or(0);
        let mut grid =
            Vec::with_capacity(content.len() + 2 * (width + 2) + content.lines().count() * 2);
        let mut height: usize = 0;
        grid.extend([b'.'].repeat(width + 2));
        for line in content.lines() {
            height += 1;
            grid.push(b'.');
            grid.extend(line.bytes());
            grid.push(b'.');
        }
        grid.extend([b'.'].repeat(width + 2));
        let mut result: usize = 0;
        loop {
            let mut removed = false;
            for x in 1..=width {
                for y in 1..=height {
                    if grid[y * (width + 2) + x] != b'@' {
                        continue;
                    }
                    let mut neigh: usize = 0;

                    neigh += if grid[(y - 1) * (width + 2) + (x - 1)] == b'@' {
                        1
                    } else {
                        0
                    };
                    neigh += if grid[(y - 1) * (width + 2) + (x + 0)] == b'@' {
                        1
                    } else {
                        0
                    };
                    neigh += if grid[(y + 1) * (width + 2) + (x - 1)] == b'@' {
                        1
                    } else {
                        0
                    };
                    neigh += if grid[(y + 0) * (width + 2) + (x - 1)] == b'@' {
                        1
                    } else {
                        0
                    };
                    neigh += if grid[(y + 1) * (width + 2) + (x + 0)] == b'@' {
                        1
                    } else {
                        0
                    };
                    neigh += if grid[(y + 0) * (width + 2) + (x + 1)] == b'@' {
                        1
                    } else {
                        0
                    };
                    neigh += if grid[(y + 1) * (width + 2) + (x + 1)] == b'@' {
                        1
                    } else {
                        0
                    };
                    neigh += if grid[(y - 1) * (width + 2) + (x + 1)] == b'@' {
                        1
                    } else {
                        0
                    };
                    if neigh < 4 {
                        result += 1;
                        removed = true;
                        grid[y * (width + 2) + x] = b'.';
                    }
                }
            }
            if !removed {
                break;
            }
        }
        Ok(result)
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
