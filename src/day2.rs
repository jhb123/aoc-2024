use std::{
    fs::File,
    io::{self, BufRead},
    ops::Sub,
};

trait Level<T> {
    fn is_monotonic(self: &Self) -> bool;
    fn safe_max_diff(self: &Self, max_diff: T) -> bool;
    fn is_safe(self: &Self, max_diff: T) -> bool {
        self.is_monotonic() && self.safe_max_diff(max_diff)
    }

    fn is_monotonic_damper(self: &Self) -> bool {
        return self.is_positive_damper() | self.is_negative_damper();
    }
    fn safe_max_diff_damper(self: &Self, max_diff: T) -> bool;

    fn is_positive_damper(self: &Self) -> bool;
    fn is_negative_damper(self: &Self) -> bool;
}

impl<T> Level<T> for Vec<T>
where
    T: Ord + Sub<Output = T> + Copy + std::fmt::Debug,
{
    fn is_monotonic(self: &Self) -> bool {
        let is_positive = self.windows(2).into_iter().all(|x| x[1] > x[0]);
        let is_negative = self.windows(2).into_iter().all(|x| x[1] < x[0]);
        return is_positive | is_negative;
    }

    fn safe_max_diff(self: &Self, max_diff: T) -> bool {
        self.windows(2)
            .into_iter()
            .map(|x| {
                if x[1] > x[0] {
                    x[1] - x[0]
                } else {
                    x[0] - x[1]
                }
            })
            .max()
            .is_some_and(|x| x <= max_diff)
    }

    fn safe_max_diff_damper(self: &Self, max_diff: T) -> bool {
        if !self.is_monotonic_damper() {
            return false;
        }
        let binding = self.iter().rev().cloned().collect::<Vec<T>>();
        let foo = match self.is_negative_damper() {
            true => &binding,
            false => self,
        };
        let bad_idx = foo
            .windows(2)
            .into_iter()
            .take_while(|&x| x[1] > x[0] && x[1] - x[0] <= max_diff)
            .count();
        if bad_idx == foo.len() - 1 {
            return true;
        };
        let a = foo
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != bad_idx)
            .map(|(_, v)| v)
            .cloned()
            .collect::<Vec<T>>()
            .windows(2)
            .into_iter()
            .all(|x| x[1] > x[0] && x[1] - x[0] <= max_diff);

        let b = foo
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != bad_idx + 1)
            .map(|(_, v)| v)
            .cloned()
            .collect::<Vec<T>>()
            .windows(2)
            .into_iter()
            .all(|x| x[1] > x[0] && x[1] - x[0] <= max_diff);
        return a | b;
    }

    fn is_positive_damper(self: &Self) -> bool {
        let bad_idx = self
            .windows(2)
            .into_iter()
            .take_while(|&x| x[1] > x[0])
            .count();
        if bad_idx == self.len() - 1 {
            return true;
        };
        // try take away the bad idx or the bad idx + 1
        let a = self
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != bad_idx)
            .map(|(_, v)| v)
            .cloned()
            .collect::<Vec<T>>()
            .windows(2)
            .into_iter()
            .all(|x| x[1] > x[0]);

        let b = self
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != bad_idx + 1)
            .map(|(_, v)| v)
            .cloned()
            .collect::<Vec<T>>()
            .windows(2)
            .into_iter()
            .all(|x| x[1] > x[0]);
        return a | b;
    }

    fn is_negative_damper(self: &Self) -> bool {
        self.iter()
            .rev()
            .cloned()
            .collect::<Vec<T>>()
            .is_positive_damper()
    }
}

pub fn solution() {
    let data = load_day_two("day2.txt");
    let safe_levels = part_one(&data);
    let safe_levels_damper = part_two(&data);
    println!("Day 2 part 1: {safe_levels}");
    println!("Day 2 part 1: {safe_levels_damper}");
}

fn part_one(data: &Vec<Vec<i64>>) -> u64 {
    data.iter()
        .fold(0, |acc, level| if level.is_safe(3) { acc + 1 } else { acc })
}

fn part_two(data: &Vec<Vec<i64>>) -> u64 {
    data.iter().fold(0, |acc, level| {
        if level.safe_max_diff_damper(3) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn load_day_two(file: &str) -> Vec<Vec<i64>> {
    let file = File::open(file).expect("missing the test data");
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut data: Vec<Vec<i64>> = Vec::with_capacity(lines.len());

    for line in lines {
        let nums = line.split(" ");
        data.push(nums.into_iter().map(|x| x.parse().unwrap()).collect());
    }

    return data;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let data = load_day_two("day2test.txt");
        assert_eq!(2, part_one(&data))
    }

    #[test]
    fn test_part_two() {
        let data = load_day_two("day2test.txt");
        assert_eq!(4, part_two(&data))
    }

    #[test]
    fn test_not_monotonic() {
        let test_cases = vec![
            vec![0, 0, 0],
            vec![0, 1, 0],
            vec![0, 1, -1],
            vec![0, 10, 10],
        ];

        for test_case in test_cases {
            assert!(!test_case.is_monotonic())
        }
    }

    #[test]
    fn test_monotonic() {
        let test_cases = vec![vec![0, 1, 2], vec![-1, -2, -3]];

        for test_case in test_cases {
            assert!(test_case.is_monotonic())
        }
    }

    #[test]
    fn safe_max_diff() {
        let test_cases = vec![
            (vec![0, 1, 2], 3),
            (vec![0, 1, 4], 3),
            (vec![-1, -2, -3], 3),
            (vec![-1, -2, -5], 3),
        ];

        for (test_case, diff) in test_cases {
            assert!(test_case.safe_max_diff(diff))
        }
    }

    #[test]
    fn not_safe_max_diff() {
        let test_cases = vec![
            (vec![0, 1, -5], 3),
            (vec![0, 1, 5], 3),
            (vec![-1, -2, -9], 3),
        ];

        for (test_case, diff) in test_cases {
            assert!(!test_case.safe_max_diff(diff))
        }
    }

    #[test]
    fn safe_tests() {
        let test_cases = vec![vec![7, 6, 4, 2, 1], vec![1, 3, 6, 7, 9]];

        for test_case in test_cases {
            assert!(test_case.is_safe(3))
        }
    }

    #[test]
    fn unsafe_tests() {
        let test_cases = vec![vec![1, 2, 7, 8, 9], vec![9, 7, 6, 2, 1]];

        for test_case in test_cases {
            assert!(!test_case.is_safe(3))
        }
    }

    #[test]
    fn test_monotonic_damper() {
        let test_cases = vec![
            vec![1, 3, 2, 4, 5],
            vec![1, 3, 3, 5, 6],
            vec![1, 0, 4, 5, 6],
            vec![-1, -3, -2, -4, -5],
        ];

        for mut test_case in test_cases {
            let res = test_case.is_monotonic_damper();
            assert!(res)
        }
    }

    #[test]
    fn test_bad_monotonic_damper() {
        let test_cases = vec![
            vec![3, 3, 3, 5, 6],
            vec![1, 2, 3, 3, 3],
            vec![1, 1, 4, 3, 3],
            vec![1, 3, 4, 3, 3],
            vec![1, 0, 4, 3, 3],
        ];

        for mut test_case in test_cases {
            println!("test case {:?}", test_case);
            let res = test_case.is_monotonic_damper();
            assert!(!res)
        }
    }

    #[test]
    fn test_safe_max_diff_damper() {
        let test_cases = vec![
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![19, 20, 21, 22, 23, 25, 26, 30],
            vec![0, 1, 2, 100, 3, 4],
            vec![0, 1, 100, 2, 3, 4],
            vec![0, 1, 2, 100, 4],
            vec![8, 11, 14, 16, 15],
            vec![-3, -100, -4, -5, -6],
        ];

        for mut test_case in test_cases {
            println!("Test case {:?}", test_case);
            let res = test_case.safe_max_diff_damper(3);
            assert!(res)
        }
    }

    #[test]
    fn test_not_safe_max_diff_damper() {
        let test_cases = vec![vec![1, 2, 7, 8, 9]];

        for mut test_case in test_cases {
            let res = test_case.safe_max_diff_damper(3);
            assert!(!res)
        }
    }

    #[test]
    fn is_positive_damper() {
        let test_cases = vec![
            vec![0, 1, 2, 3, 5, 4],
            vec![0, 1, 2, 5, 4, 5],
            vec![0, 1, 2, 100, 3, 4],
            vec![8, 11, 14, 16, 15],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![1, 2, 3],
            vec![1, 1, 2, 3, 4],
            vec![-1, 100, 2, 3, 4],
            vec![-6, -5, -4, -100, -3],
        ];

        for mut test_case in test_cases {
            let res = test_case.is_positive_damper();
            assert!(res)
        }
    }

    #[test]
    fn is_not_positive_damper() {
        let test_cases = vec![
            vec![0, 1, 2, 100, 200, 3, 4],
            // vec![100, 200, 0, 1, 2, 3, 4],
        ];

        for mut test_case in test_cases {
            let res = test_case.is_positive_damper();
            assert!(!res)
        }
    }

    #[test]
    fn is_negative_damper() {
        let test_cases = vec![
            vec![5, 4, 300, 2, 1],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![3, 2, 1],
            vec![-3, -100, -4, -5, -6],
        ];

        for mut test_case in test_cases {
            println!("test case {:?}", test_case);
            let res = test_case.is_negative_damper();
            assert!(res)
        }
    }
}
