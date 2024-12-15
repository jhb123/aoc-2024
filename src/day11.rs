// todo, add my own cache
use std::fs;

use cached::proc_macro::cached;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Key {
    number: u128,
    blinks: usize,
}

pub fn solution() {
    let data = fs::read_to_string("day11.txt").unwrap();
    let mut nums: Vec<u128> = data
        .split_ascii_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

    let mut total = 0;
    for n in nums {
        total += part_2_solution(n, 75);
    }
    println!("{total}");
}

#[cached]
fn part_2_solution(x: u128, blink: usize) -> usize {
    // recursively calculate the size of the resulting
    // array based on the rules.

    // 0 blinks, no splits
    if blink == 0 {
        return 1;
    }

    if x == 0 {
        return part_2_solution(1, blink - 1);
    } else if num_digits(&x) % 2 == 0 {
        // creates 2 branches to explore.
        let (lhs, rhs) = split_num(&x).unwrap();
        return part_2_solution(lhs, blink - 1) + part_2_solution(rhs, blink - 1);
    } else {
        return part_2_solution(x * 2024, blink - 1);
    }
}

trait Soltuion {
    fn stone_split(&self) -> Self;
}

impl Soltuion for Vec<u128> {
    fn stone_split(&self) -> Self {
        let mut new_stones = Vec::new();
        for stone in self.iter() {
            if *stone == 0 {
                new_stones.push(1);
            } else if num_digits(stone) % 2 == 0 {
                let (lhs, rhs) = split_num(stone).unwrap();
                new_stones.push(lhs);
                new_stones.push(rhs);
            } else {
                new_stones.push(stone * 2024);
            }
        }
        return new_stones;
    }
}

fn num_digits(x: &u128) -> u128 {
    if *x == 0 {
        return 1;
    }
    let mut num_digits = 0;
    let mut x = x.clone();
    while x > 0 {
        x /= 10;
        num_digits += 1;
    }
    num_digits
}

fn split_num(x: &u128) -> Result<(u128, u128), &str> {
    let n = num_digits(x);
    if n % 2 == 1 {
        return Err("Must be an even number of digits");
    } else {
        let lhs = x / 10_u128.pow(n as u32 / 2);
        let rhs = x - lhs * 10_u128.pow(n as u32 / 2); //
        return Ok((lhs, rhs));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(&1), 1)
    }

    #[test]
    fn test_num_digits_2() {
        assert_eq!(num_digits(&10), 2)
    }
    #[test]
    fn test_num_digits_3() {
        assert_eq!(num_digits(&11), 2)
    }
    #[test]
    fn test_num_digits_4() {
        assert_eq!(num_digits(&100), 3)
    }
    #[test]
    fn test_num_digits_5() {
        assert_eq!(num_digits(&9), 1)
    }
    #[test]
    fn test_num_digits_6() {
        assert_eq!(num_digits(&0), 1)
    }

    #[test]
    fn test_split() {
        assert_eq!(split_num(&11), Ok((1, 1)))
    }

    #[test]
    fn test_split2() {
        assert_eq!(split_num(&1122), Ok((11, 22)))
    }

    #[test]
    fn test_split3() {
        assert_eq!(split_num(&8989), Ok((89, 89)))
    }
    #[test]
    fn test_split4() {
        assert_eq!(split_num(&1000), Ok((10, 0)))
    }

    #[test]
    fn test_algo1() {
        let data = vec![125];
        assert_eq!(data.stone_split(), vec![253000])
    }

    #[test]
    fn test_algo2() {
        let data = vec![253000];
        assert_eq!(data.stone_split(), vec![253, 0])
    }
}
