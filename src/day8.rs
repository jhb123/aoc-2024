// transform x,y coordinats to r, theta
// start with global coordinate system.,
//
//  transformation rules,
//  xg = global coordinate (x)
//  xo = shift to antenna being centre of coordinate system
//  x = xg+xo, location relative to antenna
//  x = r cosine theta
//  y = r sine theta
//  r^2 = x^2 + y^2
//  theta = atan2 (y/x)

use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, Sub},
};

use ahash::AHashMap;

pub fn solution() {
    // step 1, obtain coordinates in global coordinates

    let f = File::open("day8test.txt").unwrap();
    let r = BufReader::new(f);
    let mut antennas = AHashMap::new();
    let mut width: i64 = 0;
    let mut height: i64 = 0;
    for (y, line) in r.lines().enumerate() {
        height += 1;
        width = line.as_ref().unwrap().len().try_into().unwrap();
        line.unwrap().chars().enumerate().for_each(|(x, c)| {
            if c.is_ascii_alphanumeric() {
                antennas.insert(
                    CartesianCoordinate::new(x.try_into().unwrap(), y.try_into().unwrap()),
                    c,
                );
            }
        });
    }
    let mut map = AntennaMap::new(antennas, width, height);
    map.calculate_anti_nodes();
    println!("{}", map);
    println!("Number of antinodes: {}", map.anti_nodes.len())
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct CartesianCoordinate {
    x: i64,
    y: i64,
}

impl CartesianCoordinate {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl From<PolarCoordinate> for CartesianCoordinate {
    fn from(value: PolarCoordinate) -> Self {
        let x: i64 = (value.r * f64::cos(value.theta)).round() as i64;
        let y: i64 = (value.r * f64::sin(value.theta)).round() as i64;
        Self::new(x, y)
    }
}

impl Sub for CartesianCoordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for CartesianCoordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::fmt::Display for CartesianCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={},y={})", self.x, self.y)
    }
}

struct PolarCoordinate {
    r: f64,
    theta: f64,
}

impl PolarCoordinate {
    fn new(r: f64, theta: f64) -> Self {
        Self { r, theta }
    }
}

impl From<CartesianCoordinate> for PolarCoordinate {
    fn from(value: CartesianCoordinate) -> Self {
        let r = f64::sqrt((value.x.pow(2) + value.y.pow(2)) as f64);
        let theta = f64::atan2(value.y as f64, value.x as f64);
        Self::new(r, theta)
    }
}

impl std::fmt::Display for PolarCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(r={},θ={})", self.r, self.theta)
    }
}

struct AntennaMap {
    width: i64,
    height: i64,
    hash_map: AHashMap<CartesianCoordinate, char>,
    anti_nodes: AHashMap<CartesianCoordinate, char>,
}

impl AntennaMap {
    fn new(hash_map: AHashMap<CartesianCoordinate, char>, width: i64, height: i64) -> Self {
        Self {
            hash_map,
            width,
            height,
            anti_nodes: AHashMap::new(),
        }
    }

    fn calculate_anti_nodes(&mut self) {
        for (&origin, val1) in &self.hash_map {
            for (&coord, val2) in &self.hash_map {
                if coord == origin {
                    continue;
                }
                if val1 == val2 {
                    let c: CartesianCoordinate = coord - origin;
                    let p: PolarCoordinate = c.into();
                    // brute force
                    for f in 0..max(self.width, self.height) {
                        let f = f as f64;
                        let anti_node_p = PolarCoordinate::new(p.r * f, p.theta);
                        let anti_node_c = CartesianCoordinate::from(anti_node_p) + origin;
                        if (0..self.width).contains(&anti_node_c.x)
                            && (0..self.height).contains(&anti_node_c.y)
                        {
                            self.anti_nodes.insert(anti_node_c, '#');
                        }
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for AntennaMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let m = CartesianCoordinate::new(col, row);
                if let Some(s) = self.hash_map.get(&m) {
                    write!(f, "{s}");
                } else if let Some(_s) = self.anti_nodes.get(&m) {
                    write!(f, "#");
                } else {
                    write!(f, ".");
                }
            }
            writeln!(f, "");
        }
        Ok(())
    }
}
