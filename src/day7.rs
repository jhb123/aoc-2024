#![allow(dead_code)]
/// this is a fairly horrible implementation of a tree

use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    mem,
    ops::{Add, Mul},
    str::FromStr,
};

pub fn solution() {
    let equations = load_data("day7.txt");
    let mut tot = 0;
    for eq in equations {
        if eq.data.tree_search(eq.target) {
            tot += eq.target
        }
    }
    println!("day 7 part 1: {tot}");

    let equations = load_data("day7.txt");
    let mut tot2 = 0;
    for eq in equations {
        if eq.data.tree_search2(eq.target) {
            tot2 += eq.target
        }
    }
    println!("day 7 part 2: {tot2}");
}

struct Equation {
    target: u128,
    data: Vec<u128>,
}
impl Equation {
    fn new(target: u128, data: Vec<u128>) -> Self {
        Self { target, data }
    }
}

fn load_data(path: &str) -> Vec<Equation> {
    let f = File::open(path).unwrap();
    let r = BufReader::new(f);
    let mut equations = vec![];
    r.lines().for_each(|line| {
        let data = line.unwrap();
        let mut spl = data.split(": ");
        let target: u128 = spl.next().unwrap().parse().unwrap();
        let nums: Vec<u128> = spl
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        equations.push(Equation::new(target, nums));
    });
    equations
}

#[derive(Debug)]
struct DFS<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + PartialEq,
{
    data: Vec<T>,
    val: T,
    left_add: Option<Box<DFS<T>>>,
    right_mult: Option<Box<DFS<T>>>,
}

impl<T> DFS<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + PartialEq,
{
    fn new(data: Vec<T>, val: T) -> Self {
        Self {
            data: data,
            val: val,
            left_add: None,
            right_mult: None,
        }
    }

    fn count_size(&self) -> usize {
        let mut size = 0;
        if self.left_add.is_none() && self.right_mult.is_none() {
            return 1;
        } else {
            size += self.left_add.as_ref().unwrap().count_size()
                + self.right_mult.as_ref().unwrap().count_size();
            return size;
        }
    }

    fn search(&mut self, target: T) -> bool {
        if self.data.len() == 0 && self.val == target {
            return true;
        } else if self.data.len() == 0 && self.val != target {
            return false;
        } else {
            let _ = mem::replace(
                &mut self.right_mult,
                Some(Box::new(DFS::new(
                    self.data[1..self.data.len()].to_vec(),
                    self.val * self.data[0],
                ))),
            );
            let _ = mem::replace(
                &mut self.left_add,
                Some(Box::new(DFS::new(
                    self.data[1..self.data.len()].to_vec(),
                    self.val + self.data[0],
                ))),
            );

            if self.left_add.as_mut().unwrap().search(target) {
                return true;
            } else {
                return self.right_mult.as_mut().unwrap().search(target);
            }
        }
    }
}

struct DFS2<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + PartialEq + FromStr + std::fmt::Display,
{
    data: Vec<T>,
    val: T,
    left_add: Option<Box<DFS2<T>>>,
    right_mult: Option<Box<DFS2<T>>>,
    middle_concat: Option<Box<DFS2<T>>>,
}

impl<T> DFS2<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + PartialEq + FromStr + std::fmt::Display,
{
    fn new(data: Vec<T>, val: T) -> Self {
        Self {
            data: data,
            val: val,
            left_add: None,
            right_mult: None,
            middle_concat: None,
        }
    }

    fn search(&mut self, target: T) -> bool {
        if self.data.len() == 0 && self.val == target {
            return true;
        } else if self.data.len() == 0 && self.val != target {
            return false;
        } else {
            let _ = mem::replace(
                &mut self.right_mult,
                Some(Box::new(DFS2::new(
                    self.data[1..self.data.len()].to_vec(),
                    self.val * self.data[0],
                ))),
            );
            let _ = mem::replace(
                &mut self.left_add,
                Some(Box::new(DFS2::new(
                    self.data[1..self.data.len()].to_vec(),
                    self.val + self.data[0],
                ))),
            );
            if let Ok(r) = format!("{}{}", self.val, self.data[0]).parse() {
                let _ = mem::replace(
                    &mut self.middle_concat,
                    Some(Box::new(DFS2::new(
                        self.data[1..self.data.len()].to_vec(),
                        r,
                    ))),
                );
            }

            if self.left_add.as_mut().unwrap().search(target) {
                return true;
            } else if self.right_mult.as_mut().unwrap().search(target) {
                return true;
            } else if let Some(dfs) = self.middle_concat.as_mut() {
                return dfs.as_mut().search(target);
            } else {
                return false;
            }
        }
    }
}

trait TreeSearch<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + PartialEq + FromStr + Display,
{
    fn tree_search(&self, target: T) -> bool;
    fn tree_search2(&self, target: T) -> bool;
}

impl<T> TreeSearch<T> for Vec<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + PartialEq + FromStr + Display,
{
    fn tree_search(&self, target: T) -> bool {
        let mut dfs = DFS::new(self[1..self.len()].to_vec(), self[0]);
        dfs.search(target)
    }

    fn tree_search2(&self, target: T) -> bool {
        let mut dfs = DFS2::new(self[1..self.len()].to_vec(), self[0]);
        dfs.search(target)
    }
}
