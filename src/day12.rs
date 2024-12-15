use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

pub fn solution() {
    let f = Farm::load_data("day12.txt");
    let _p = f.calculate_price();
    // f.display();

    let _sf = f.find_region(1, 1);

    // sf.display();
    // let a = sf.find_area();
    // let b = sf.find_perimeter();
    // let corners = sf.count_corners(Some((1,1)));
    let price = f.calculate_price2();
    println!("{}", price)
}

struct Farm {
    width: i32,
    height: i32,
    data: Vec<char>,
    special_coords: Option<HashSet<(usize, usize)>>,
}

impl Farm {
    fn new(width: i32, height: i32, data: Vec<char>) -> Self {
        Self {
            width,
            height,
            data,
            special_coords: None,
        }
    }

    fn calculate_price(&self) -> usize {
        let mut checked_coords: HashSet<(usize, usize)> = HashSet::new();
        let mut total_price = 0;
        for i in 1..(self.height - 1) {
            for j in 1..(self.width - 1) {
                if checked_coords.contains(&(i as usize, j as usize)) {
                    continue;
                } else {
                    let sf = self.find_region(i as usize, j as usize);
                    // sf.display();
                    // println!("");
                    let a = sf.find_area();
                    let b = sf.find_perimeter();
                    let price = a.unwrap() * b.unwrap();
                    let cc = sf.special_coords.unwrap();
                    checked_coords = checked_coords.union(&cc).cloned().collect();
                    total_price += price;
                }
            }
        }
        return total_price;
    }

    fn calculate_price2(&self) -> usize {
        let mut checked_coords: HashSet<(usize, usize)> = HashSet::new();
        let mut total_price = 0;
        for i in 1..(self.height - 1) {
            for j in 1..(self.width - 1) {
                if checked_coords.contains(&(i as usize, j as usize)) {
                    continue;
                } else {
                    let sf = self.find_region(i as usize, j as usize);
                    // sf.display();
                    // println!("");
                    let a = sf.find_area();
                    let corners = sf.count_corners(None);
                    let edges = corners;
                    let price = a.unwrap() * edges;
                    let cc = sf.special_coords.unwrap();
                    checked_coords = checked_coords.union(&cc).cloned().collect();
                    total_price += price;
                }
            }
        }
        return total_price;
    }

    fn count_corners(&self, coord: Option<(usize,usize)>) -> usize {
        println!("");
        self.display();
        let r = match coord {
            Some(r) => r,
            None => {
                let region = self.special_coords.as_ref().unwrap();
                region.iter().cloned().next().unwrap()   
            },
        };
        

        let c = self.get_ij(r.0, r.1).unwrap();

        let mut concave_count = 0;
        //    |
        //  - x
        for i in 1..(self.height as usize - 1) {
            for j in 1..(self.width as usize - 1) {
                let c0 = self.get_ij(i, j).unwrap();
                let c1 = self.get_ij(i - 1, j).unwrap();
                let c2 = self.get_ij(i, j - 1).unwrap();
                if c1 != c && c2 != c && c0==c {
                    concave_count += 1
                }
            }
        }
        //    x -
        //    |
        for i in 1..(self.height as usize - 1) {
            for j in 1..(self.width as usize - 1) {
                let c0 = self.get_ij(i, j).unwrap();
                let c1 = self.get_ij(i + 1, j).unwrap();
                let c2 = self.get_ij(i, j + 1).unwrap();
                if c1 != c && c2 != c && c0==c{
                    concave_count += 1
                }
            }
        }
        //  - x
        //    |
        for i in 1..(self.height as usize - 1) {
            for j in 1..(self.width as usize - 1) {
                let c0 = self.get_ij(i, j).unwrap();
                let c1 = self.get_ij(i - 1, j).unwrap();
                let c2 = self.get_ij(i, j + 1).unwrap();
                if c1 != c && c2 != c && c0==c{
                    concave_count += 1
                }
            }
        }
        //    |
        //    x -
        for i in 1..(self.height as usize - 1) {
            for j in 1..(self.width as usize - 1) {
                let c0 = self.get_ij(i, j).unwrap();
                let c1 = self.get_ij(i + 1, j).unwrap();
                let c2 = self.get_ij(i, j - 1).unwrap();
                if c1 != c && c2 != c && c0==c{
                    concave_count += 1
                }
            }
        }

        
        let mut convex_count = 0;
        //    x =
        //    = .
        for i in 1..(self.height as usize - 1) {
            for j in 1..(self.width as usize - 1) {
                let c0 = self.get_ij(i, j).unwrap();
                let c1 = self.get_ij(i + 1, j + 1).unwrap();
                let c2 = self.get_ij(i + 1, j).unwrap();
                let c3 = self.get_ij(i, j + 1).unwrap();
                if c1 != c  && c2==c  && c3==c && c0==c{
                    convex_count += 1
                }
            }
        }

        for i in 1..(self.height as usize - 1) {
            for j in 1..(self.width as usize - 1) {
                let c0 = self.get_ij(i, j).unwrap();
                let c1 = self.get_ij(i - 1, j - 1).unwrap();
                let c2 = self.get_ij(i - 1, j).unwrap();
                let c3 = self.get_ij(i, j - 1).unwrap();
                if c1 != c  && c2==c  && c3==c && c0==c{
                    convex_count += 1
                }
            }
        }
        for i in 1..(self.height as usize - 1) {
            for j in 1..(self.width as usize - 1) {
                let c0 = self.get_ij(i, j).unwrap();
                let c1 = self.get_ij(i + 1, j - 1).unwrap();
                let c2 = self.get_ij(i + 1, j).unwrap();
                let c3 = self.get_ij(i, j - 1).unwrap();
                if c1 != c  && c2==c  && c3==c && c0==c{
                    convex_count += 1
                }
            }
        }
        for i in 1..(self.height as usize - 1) {
            for j in 1..(self.width as usize - 1) {
                let c0 = self.get_ij(i, j).unwrap();
                let c1 = self.get_ij(i - 1, j + 1).unwrap();
                let c2 = self.get_ij(i - 1, j).unwrap();
                let c3 = self.get_ij(i, j + 1).unwrap();
                if c1 != c  && c2==c  && c3==c && c0==c{
                    convex_count += 1
                }
            }
        }

        return concave_count + convex_count;
    }

    fn find_region(&self, i: usize, j: usize) -> Farm {
        let mut checked_coords: HashSet<(usize, usize)> = HashSet::new();

        let mut to_check_stack: Vec<(usize, usize)> = vec![(i, j)];
        let region_char = self.get_ij(i, j).unwrap();

        while let Some(coord) = to_check_stack.pop() {
            if checked_coords.contains(&coord) {
                continue;
            } else {
                let neighbours = self.find_neighbours(coord.0, coord.1);
                for neighbour in neighbours {
                    if neighbour.1 == region_char {
                        if !checked_coords.contains(&(neighbour.0 .0, neighbour.0 .1)) {
                            to_check_stack.push((neighbour.0 .0, neighbour.0 .1));
                        }
                    }
                }
                checked_coords.insert((coord.0, coord.1));
            }
        }

        let mut sub_farm_data = vec![];
        for i in 0..self.height {
            for j in 0..self.width {
                if let Some(coord) = checked_coords.get(&(i as usize, j as usize)) {
                    let c = self.get_ij(coord.0, coord.1).unwrap();
                    sub_farm_data.push(c);
                } else {
                    sub_farm_data.push('.');
                }
            }
        }
        Farm {
            width: self.width,
            height: self.height,
            data: sub_farm_data,
            special_coords: Some(checked_coords),
        }
    }

    fn find_neighbours(&self, i: usize, j: usize) -> Vec<((usize, usize), char)> {
        let mut neighbours = vec![];
        neighbours.push(((i - 1, j), self.get_ij(i - 1, j).unwrap()));
        neighbours.push(((i, j - 1), self.get_ij(i, j - 1).unwrap()));
        neighbours.push(((i, j + 1), self.get_ij(i, j + 1).unwrap()));
        neighbours.push(((i + 1, j), self.get_ij(i + 1, j).unwrap()));
        return neighbours;
    }

    fn find_perimeter(&self) -> Result<usize, &str> {
        if let Some(_c) = &self.special_coords {
            let (i, j) = self
                .special_coords
                .as_ref()
                .unwrap()
                .iter()
                .cloned()
                .next()
                .unwrap();
            let mut checked_coords: HashSet<(usize, usize)> = HashSet::new();
            let mut to_check_stack: Vec<(usize, usize)> = vec![(i, j)];
            let region_char = self.get_ij(i, j).unwrap();
            let mut total_edges = 0;

            while let Some(coord) = to_check_stack.pop() {
                if checked_coords.contains(&coord) {
                    continue;
                } else {
                    let neighbours = self.find_neighbours(coord.0, coord.1);
                    let mut num_edges = 4;
                    for neighbour in neighbours {
                        if neighbour.1 == region_char {
                            if !checked_coords.contains(&(neighbour.0 .0, neighbour.0 .1)) {
                                to_check_stack.push((neighbour.0 .0, neighbour.0 .1));
                            }
                            num_edges -= 1;
                        }
                    }
                    checked_coords.insert((coord.0, coord.1));
                    total_edges += num_edges
                }
            }
            Ok(total_edges)
        } else {
            Err("This farm has no special region to analyse.")
        }
    }

    fn find_area(&self) -> Result<usize, &str> {
        if let Some(c) = &self.special_coords {
            return Ok(c.len());
        } else {
            Err("This farm has no special region to analyse.")
        }
    }

    fn get_ij(&self, i: usize, j: usize) -> Option<char> {
        return self.data.get(j + i * self.width as usize).copied();
    }

    fn display(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if let Some(c) = self.get_ij(i as usize, j as usize) {
                    if c == '.' {
                        print!("\x1b[2m\x1b[90m{c}\x1b[0m")

                    } else {
                        print!("{c}")
                    }
                }
            }
            print!("\n")
        }
    }

    fn load_data(file_name: &str) -> Self {
        let padding = 1;

        let file = File::open(file_name).unwrap();
        let reader = io::BufReader::new(&file);
        let height = reader.lines().count();

        let file = File::open(file_name).unwrap();
        let reader = io::BufReader::new(&file);
        let first_line = reader.lines().next().unwrap().unwrap();
        let width = first_line.len();

        println!("{width}x{height}");

        let mut data = vec![];
        for _i in 0..padding {
            let mut f = vec!['.'; width + 2 * padding];
            data.append(&mut f);
        }

        let file = File::open(file_name).unwrap();
        let mut lines = io::BufReader::new(&file).lines();

        while let Some(line) = lines.next() {
            let l = line.unwrap();
            let mut line_data = vec![];
            line_data.append(&mut vec!['.'; padding]);
            let mut chars: Vec<char> = l.chars().collect();
            line_data.append(&mut chars);
            line_data.append(&mut vec!['.'; padding]);
            data.append(&mut line_data);
        }

        for _i in 0..padding {
            let mut f = vec!['.'; width + 2 * padding];
            data.append(&mut f);
        }
        Farm::new(
            (width + 2 * padding).try_into().unwrap(),
            (height + 2 * padding).try_into().unwrap(),
            data,
        )
    }
}
