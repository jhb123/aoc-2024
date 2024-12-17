/// this wasn't the best puzzle ever... first half was easy
/// second half was weird lateral thinking from the internet
use std::{collections::HashSet, fs};

use regex::Regex;

const WIDTH: i64 = 101; // 101, 11
const HEIGHT: i64 = 103; // 103, 7

pub fn solution() {
    let robots = load_data("day14.txt");

    let mut positions = vec![];
    for robot in robots {
        positions.push(calculate_position(&robot, 100))
    }

    let mut c0 = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;

    for (x, y) in positions {
        println!("position ({},{})", x, y);
        if x < WIDTH / 2 && y < HEIGHT / 2 {
            c0 += 1
        } else if x < WIDTH / 2 && y > HEIGHT / 2 {
            c1 += 1
        } else if x > WIDTH / 2 && y < HEIGHT / 2 {
            c2 += 1
        } else if x > WIDTH / 2 && y > HEIGHT / 2 {
            c3 += 1
        }
    }
    println!("part 1: {}, {c0},{c1},{c2},{c3}", c0 * c1 * c2 * c3);

    let robots = load_data("day14.txt");

    let initial_pos: Vec<(i64, i64)> = robots.iter().map(|r| (r.x, r.y)).collect();

    for i in 0..10000 {
        let mut positions = vec![];
        for robot in robots.iter().cloned() {
            positions.push(calculate_position(&robot, i))
        }
        let mut set: HashSet<_> = HashSet::from_iter(positions.iter());
        if set.len() == robots.len() {
            render(positions);
            println!("step {i}");
        }

        // if initial_pos == positions && i !=0 {
        //     break;
        // }
        // thread::sleep_ms(100);
    }
}

fn render(positions: Vec<(i64, i64)>) {
    let positions: HashSet<_> = HashSet::from_iter(positions.iter());

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            if positions.contains(&(col, row)) {
                print!("*")
            } else {
                print!(" ")
            }
        }
        println!()
    }
}

#[derive(Clone)]
struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

fn calculate_position(robot: &Robot, time: i64) -> (i64, i64) {
    let mut x = (robot.x + robot.vx * time) % WIDTH;
    let mut y = (robot.y + robot.vy * time) % HEIGHT;
    if x < 0 {
        x += WIDTH
    };
    if y < 0 {
        y += HEIGHT
    };

    (x, y)
}

fn load_data(data: &str) -> Vec<Robot> {
    let raw = fs::read_to_string(data).unwrap();
    let re = Regex::new(r"p=(\d+),(\d+) v=(\-?\d+),(\-?\d+)").unwrap();

    let result = re.captures_iter(&raw).map(|c| c.extract());
    let mut robots = vec![];
    for (_, [x, y, vx, vy]) in result {
        let r = Robot {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            vx: vx.parse().unwrap(),
            vy: vy.parse().unwrap(),
        };
        robots.push(r);
    }
    robots
}
