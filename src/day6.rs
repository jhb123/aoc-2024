// Optimisations include speeding up the hashing

use ahash::AHashSet;
// use fxhash::FxHashSet;
use std::{
    fs::File,
    io::{self, BufRead},
    time::Instant,
    usize,
    hash::Hash
};

type Coordinate = (i64, i64);

pub fn solution() {
    let mut lab = Lab::new("day6.txt");
    while lab.guard_in_map() {
        lab.step();
    }
    let visited = lab.guard.visited.len() - 1;
    println!("Day 6 part 1: {visited}");
    lab.reset_guard();

    let mut new_obs = AHashSet::with_capacity(2000);
    let start = Instant::now(); // Start the timer
    while lab.guard_in_map() {
        if let Some(x) = lab.look_ahead() {
            new_obs.insert(x);
        }
        lab.step();
    }
    let duration = start.elapsed(); // End the timer

    println!("Day 6 part 2: {}", new_obs.len());
    println!("Part 2 execution time: {:.2?}", duration);
}

struct Lab {
    width: i64,
    height: i64,
    obsticles: AHashSet<Coordinate>,
    guard: Guard,
    block_count: usize,
    initial_pos: Coordinate,
}

impl Lab {
    fn new(file: &str) -> Self {
        // make a builder
        // let data = fs::read_to_string(file).unwrap();
        // let width = data.chars().take_while(|&c| c != '\n').count() + 1;
        let f = File::open(file).unwrap();
        let lines = io::BufReader::new(f).lines();
        let mut width = 0;
        let mut obsticles: AHashSet<Coordinate> = AHashSet::new();
        let mut guard: Guard = Guard::default();
        let height = lines
            .enumerate()
            .map(|(row, l)| {
                let characters = l.unwrap(); //.into_bytes();
                width = characters.len(); // could be less dumb
                for (col, char) in characters.chars().enumerate() {
                    match char {
                        '.' => (),
                        '#' => {
                            obsticles.insert((row.try_into().unwrap(), col.try_into().unwrap()));
                        }
                        '^' => {
                            guard = Guard::new((row.try_into().unwrap(), col.try_into().unwrap()))
                        }
                        _ => (),
                    }
                }
            })
            .count();

        Self {
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
            obsticles,
            guard: guard.clone(),
            block_count: 0,
            initial_pos: guard.position.clone(),
        }
    }

    fn step(&mut self) {
        let p = self.guard.position;

        let tentative_step = match self.guard.direction {
            Direction::North => (p.0 - 1, p.1),
            Direction::West => (p.0, p.1 + 1),
            Direction::South => (p.0 + 1, p.1),
            Direction::East => (p.0, p.1 - 1),
        };
        if self.obsticles.contains(&tentative_step) {
            self.guard.direction = self.guard.direction.next();
            self.guard
                .visited2
                .insert((self.guard.position, self.guard.direction));
        } else {
            self.guard.position = tentative_step;
            self.guard.steps += 1;
            self.guard.visited.insert(tentative_step);
            self.guard
                .visited2
                .insert((tentative_step, self.guard.direction));
        }
    }

    fn look_ahead(&self) -> Option<Coordinate> {
        // no idea why this doesn't work

        // get the current location
        let init_pos = self.guard.position;
        let init_direction = self.guard.direction;

        let mut new_guard = self.guard.clone();
        let mut new_obs = self.obsticles.clone();

        // add a new obstical at the current location + step.
        let new_obstical = match init_direction {
            Direction::North => (init_pos.0 - 1, init_pos.1),
            Direction::West => (init_pos.0, init_pos.1 + 1),
            Direction::South => (init_pos.0 + 1, init_pos.1),
            Direction::East => (init_pos.0, init_pos.1 - 1),
        };

        if new_guard.visited.contains(&new_obstical) {
            // cannot block the path to this point otherwise
            // gaurd can't reach it
            return None;
        }

        if !self.coord_in_map(&new_obstical) {
            // this is already an obstical so you cant add it
            return None;
        }

        if !new_obs.insert(new_obstical) {
            // if the obsticle isn't new, return nothing.
            return None;
        }

        // detect an infinite loop by checking where you've already been
        // set up where you've alreay been
        // let mut visited_coords = AHashSet::new();

        // do the walking algo
        while self.coord_in_map(&new_guard.position) {
            let tentative_step = match new_guard.direction {
                Direction::North => (new_guard.position.0 - 1, new_guard.position.1),
                Direction::West => (new_guard.position.0, new_guard.position.1 + 1),
                Direction::South => (new_guard.position.0 + 1, new_guard.position.1),
                Direction::East => (new_guard.position.0, new_guard.position.1 - 1),
            };
            if new_obs.contains(&tentative_step) {
                new_guard.direction = new_guard.direction.next();
            } else {
                new_guard.position = tentative_step;
                new_guard.steps += 1;
            }

            if !new_guard
                .visited2
                .insert((new_guard.position, new_guard.direction))
            {
                return Some(new_obstical);
            }
        }
        None
    }

    fn reset_guard(&mut self) {
        self.guard = Guard::new(self.initial_pos);
    }

    fn draw(&self) {
        println!();
        for c in 0..self.height {
            // print!("{} ",c);
            for r in 0..self.width {
                if self.obsticles.contains(&(c, r)) {
                    print!("#")
                } else if self.guard.position == (c, r) {
                    print!("{}", self.guard)
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }

    fn draw2(&self, new_g: Coordinate, new_obs: &AHashSet<Coordinate>) {
        // only works nicely for single digit num of rows/cols
        for c in 0..self.height {
            for r in 0..self.width {
                if self.obsticles.contains(&(c, r)) {
                    print!("#")
                } else if new_obs.contains(&(c, r)) {
                    print!(".")
                } else if self.guard.position == (c, r) {
                    print!("{}", self.guard)
                } else if new_g == (c, r) {
                    print!("G")
                } else if self.guard.visited.contains(&(c, r)) {
                    print!("x")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
        println!()
    }

    fn guard_in_map(&self) -> bool {
        self.coord_in_map(&self.guard.position)
    }

    fn coord_in_map(&self, coord: &Coordinate) -> bool {
        (0..self.width).contains(&coord.1) && (0..self.height).contains(&coord.0)
    }

    fn show_stats(&self) {
        println!(
            "Guard steps: {}. Position {:?}. Unique visits {}. Block count {}",
            self.guard.steps,
            self.guard.position,
            self.guard.visited.len(),
            self.block_count
        )
    }
}

#[derive(Default, Clone)]
struct Guard {
    position: Coordinate,
    direction: Direction,
    steps: usize,
    visited: AHashSet<Coordinate>,
    visited2: AHashSet<(Coordinate, Direction)>,
}

impl Guard {
    fn new(position: Coordinate) -> Self {
        let mut h = AHashSet::with_capacity(2000);
        let mut h2 = AHashSet::with_capacity(2000);

        h.insert(position);
        h2.insert((position, Direction::North));

        Self {
            position: position,
            direction: Direction::North,
            steps: 0,
            visited: h,
            visited2: h2,
        }
    }
}

impl std::fmt::Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::North => write!(f, "\x1b[1;32m^\x1b[0m"),
            Direction::West => write!(f, "\x1b[1;32m>\x1b[0m"),
            Direction::South => write!(f, "\x1b[1;32mv\x1b[0m"),
            Direction::East => write!(f, "\x1b[1;32m<\x1b[0m"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Copy, Debug)]
#[repr(u8)] // Might speed up hashing?
enum Direction {
    North = 0,
    West = 1,
    South = 2,
    East = 3,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

impl Direction {
    fn next(&mut self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}
