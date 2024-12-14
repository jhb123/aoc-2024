/// This solution could be optimised further. many times
/// different starting locations can have join to trails
/// which you've already calculated the root for.
/// However, this solution already seems fast enough and
/// modifying this tree to be a graph and caching
/// certain parts of the calculation would be quite a
/// bit of effort!
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution() {
    let trail_map = TrailMap::new("day10.txt");

    // let n = trail_map.data.get(&Coordinate::new(2, 2))
    let mut total = 0;
    for x in 0..trail_map.width {
        for y in 0..trail_map.height {
            let c = Coordinate::new(x.try_into().unwrap(), y.try_into().unwrap());
            let node = trail_map.data.get(&c).unwrap();
            if node.elevation == 0 {
                total += trail_map.find_trail(c);
            }
        }
    }

    let mut total_2 = 0;
    for x in 0..trail_map.width {
        for y in 0..trail_map.height {
            let c = Coordinate::new(x.try_into().unwrap(), y.try_into().unwrap());
            let node = trail_map.data.get(&c).unwrap();
            if node.elevation == 0 {
                total_2 += trail_map.find_trail_2(c);
            }
        }
    }
    println!("Total score part 1: {total}");
    println!("Total score part 2: {total_2}");
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct TrailMap {
    data: HashMap<Coordinate, TrailNode>,
    width: usize,
    height: usize,
}

impl TrailMap {
    fn new(file: &str) -> Self {
        let mut data = HashMap::new();
        let f = File::open(file).unwrap();
        let lines = BufReader::new(f).lines();
        let mut height = 0;
        let mut width = 0;

        // first pass to populate everywhere in map
        for (row, line) in lines.enumerate() {
            height += 1;
            let s = line.unwrap();
            width = s.len();
            for (col, character) in s.char_indices() {
                let elevation = character.to_digit(10).unwrap();
                let coordinate = Coordinate::new(col.try_into().unwrap(), row.try_into().unwrap());
                let node = TrailNode::new(&elevation, &coordinate);
                data.insert(coordinate, node);
            }
        }

        Self {
            data,
            width: width,
            height: height,
        }
    }

    fn find_trail(&self, coordinate: Coordinate) -> i32 {
        let mut score = 0;
        if let Some(start_node) = self.data.clone().get_mut(&coordinate) {
            start_node.populate_graph(&self);
            // n.show(self);
            let visited = start_node.get_trial_coordinates();
            for node in visited {
                let x = self.data.get(node).unwrap();
                if x.elevation == 9 {
                    score += 1;
                }
            }
        }
        return score;
    }

    fn find_trail_2(&self, coordinate: Coordinate) -> usize {
        let mut score = 0;
        if let Some(start_node) = self.data.clone().get_mut(&coordinate) {
            start_node.populate_graph(&self);
            // n.show(self);
            match start_node.get_trial_coordinates_2() {
                Some(x) => return x,
                None => return 0,
            }
        } else {
            return 0; // probably dont need this if else.
        }
    }
}

impl Display for TrailMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let coordinate = Coordinate::new(col.try_into().unwrap(), row.try_into().unwrap());
                let node = self.data.get(&coordinate).unwrap();
                if let Err(e) = write!(f, "{}", node.elevation) {
                    return Err(e);
                }
            }
            println!("")
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct TrailNode {
    elevation: u32,
    coordinate: Coordinate,
    north: Box<Option<TrailNode>>,
    east: Box<Option<TrailNode>>,
    south: Box<Option<TrailNode>>,
    west: Box<Option<TrailNode>>,
}

impl TrailNode {
    fn new(elevation: &u32, coordinate: &Coordinate) -> Self {
        Self {
            elevation: *elevation,
            coordinate: *coordinate,
            north: Box::new(None),
            east: Box::new(None),
            west: Box::new(None),
            south: Box::new(None),
        }
    }

    fn populate_graph(&mut self, map: &TrailMap) {
        let north = Coordinate::new(self.coordinate.x, self.coordinate.y - 1);
        let east = Coordinate::new(self.coordinate.x + 1, self.coordinate.y);
        let south = Coordinate::new(self.coordinate.x, self.coordinate.y + 1);
        let west = Coordinate::new(self.coordinate.x - 1, self.coordinate.y);

        // fill up the neighbour data
        if let Some(node) = map.data.get(&north) {
            if node.elevation == self.elevation + 1 {
                self.north = Box::new(Some(node.clone()));
            }
        }
        if let Some(node) = map.data.get(&east) {
            if node.elevation == self.elevation + 1 {
                self.east = Box::new(Some(node.clone()));
            }
        }
        if let Some(node) = map.data.get(&south) {
            if node.elevation == self.elevation + 1 {
                self.south = Box::new(Some(node.clone()));
            }
        }
        if let Some(node) = map.data.get(&west) {
            if node.elevation == self.elevation + 1 {
                self.west = Box::new(Some(node.clone()));
            }
        }

        // go on adventure!
        if let Some(node) = self.north.as_mut() {
            if node.elevation == self.elevation + 1 {
                node.populate_graph(map);
            }
        };
        if let Some(node) = self.east.as_mut() {
            if node.elevation == self.elevation + 1 {
                node.populate_graph(map);
            }
        };
        if let Some(node) = self.south.as_mut() {
            if node.elevation == self.elevation + 1 {
                node.populate_graph(map);
            }
        };
        if let Some(node) = self.west.as_mut() {
            if node.elevation == self.elevation + 1 {
                node.populate_graph(map);
            }
        };
    }

    fn get_trial_coordinates(&self) -> HashSet<&Coordinate> {
        let mut visited = HashSet::new();
        visited.insert(&self.coordinate);
        if let Some(n) = self.north.as_ref() {
            visited = visited.union(&n.get_trial_coordinates()).cloned().collect();
        }

        if let Some(n) = self.east.as_ref() {
            visited = visited.union(&n.get_trial_coordinates()).cloned().collect();
        }

        if let Some(n) = self.south.as_ref() {
            visited = visited.union(&n.get_trial_coordinates()).cloned().collect();
        }

        if let Some(n) = self.west.as_ref() {
            visited = visited.union(&n.get_trial_coordinates()).cloned().collect();
        }

        return visited;
    }

    fn get_trial_coordinates_2(&self) -> Option<usize> {
        if self.elevation == 9 {
            return Some(1);
        }
        let mut visited = None;

        if let Some(n) = self.north.as_ref() {
            if let Some(x) = n.get_trial_coordinates_2() {
                if let Some(y) = visited {
                    visited = Some(x + y)
                } else {
                    visited = Some(x)
                }
            }
        }

        if let Some(n) = self.east.as_ref() {
            if let Some(x) = n.get_trial_coordinates_2() {
                if let Some(y) = visited {
                    visited = Some(x + y)
                } else {
                    visited = Some(x)
                }
            }
        }

        if let Some(n) = self.south.as_ref() {
            if let Some(x) = n.get_trial_coordinates_2() {
                if let Some(y) = visited {
                    visited = Some(x + y)
                } else {
                    visited = Some(x)
                }
            }
        }

        if let Some(n) = self.west.as_ref() {
            if let Some(x) = n.get_trial_coordinates_2() {
                if let Some(y) = visited {
                    visited = Some(x + y)
                } else {
                    visited = Some(x)
                }
            }
        }

        return visited;
    }

    fn show(&self, map: &TrailMap) {
        let visited = self.get_trial_coordinates();
        for row in 0..map.height {
            for col in 0..map.width {
                let coordinate = Coordinate::new(col.try_into().unwrap(), row.try_into().unwrap());
                if let Some(&n) = visited.get(&coordinate) {
                    let node = map.data.get(&coordinate).unwrap();
                    print!("\x1b[41;37m{}\x1b[0m", node.elevation);
                } else {
                    let node = map.data.get(&coordinate).unwrap();
                    print!("{}", node.elevation);
                }
            }
            println!("")
        }
    }
}
