use std::{
    fs::{self, read_to_string, File},
    io::{self, BufRead},
};

struct WordGrid {
    data: String,
    width: usize,
}

impl WordGrid {
    pub fn new(file: &str) -> Self {
        let data = fs::read_to_string(file).unwrap();
        let width = data.chars().take_while(|&c| c != '\n').count() + 1;
        // data = data.replace("\n", "");
        Self { data, width }
    }

    fn find_all(&self) -> usize {
        let mut count = 0;
        for i in 0..self.data.len() {
            if self.check_horizontal(i) {
                println!("Found h");
                count += 1
            }
            if self.check_horizontalb(i) {
                println!("Found hb");
                count += 1
            }
            if self.check_vertical(i) {
                println!("Found v");
                count += 1
            }
            if self.check_verticalb(i) {
                println!("Found vb");
                count += 1
            }
            if self.check_diagonal_1(i) {
                println!("Found d1");
                count += 1
            }
            if self.check_diagonal_1b(i) {
                println!("Found d1b");
                count += 1
            }
            if self.check_diagonal_2(i) {
                println!("Found d2");
                count += 1
            }
            if self.check_diagonal_2b(i) {
                println!("Found d2b");
                count += 1
            }
        }
        return count;
    }

    fn find_all_horizontal(&self) -> usize {
        let mut count = 0;
        for i in 0..self.data.len() {
            if self.check_horizontal(i) {
                count += 1
            }
        }
        return count;
    }

    fn find_all_vertical(&self) -> usize {
        let mut count = 0;
        for i in 0..self.data.len() {
            if self.check_vertical(i) {
                count += 1
            }
        }
        return count;
    }

    fn check_horizontal(&self, idx: usize) -> bool {
        self.data
            .chars()
            .skip(idx)
            .step_by(1)
            .take(4)
            .collect::<String>()
            == "XMAS"
    }

    fn check_horizontalb(&self, idx: usize) -> bool {
        self.data
            .chars()
            .rev()
            .skip(idx)
            .step_by(1)
            .take(4)
            .collect::<String>()
            == "XMAS"
    }

    fn check_vertical(&self, idx: usize) -> bool {
        self.data
            .chars()
            .skip(idx)
            .step_by(self.width)
            .take(4)
            .collect::<String>()
            == "XMAS"
    }

    fn check_verticalb(&self, idx: usize) -> bool {
        self.data
            .chars()
            .rev()
            .skip(idx)
            .step_by(self.width)
            .take(4)
            .collect::<String>()
            == "XMAS"
    }

    fn check_diagonal_1(&self, idx: usize) -> bool {
        self.data
            .chars()
            .skip(idx)
            .step_by(self.width - 1)
            .take(4)
            .collect::<String>()
            == "XMAS"
    }

    fn check_diagonal_1b(&self, idx: usize) -> bool {
        self.data
            .chars()
            .rev()
            .skip(idx)
            .step_by(self.width - 1)
            .take(4)
            .collect::<String>()
            == "XMAS"
    }

    fn check_diagonal_2(&self, idx: usize) -> bool {
        self.data
            .chars()
            .skip(idx)
            .step_by(self.width + 1)
            .take(4)
            .collect::<String>()
            == "XMAS"
    }

    fn check_diagonal_2b(&self, idx: usize) -> bool {
        self.data
            .chars()
            .rev()
            .skip(idx)
            .step_by(self.width + 1)
            .take(4)
            .collect::<String>()
            == "XMAS"
    }
}

pub fn solution() {}

fn load_test_data(file: &str) -> String {
    // let file = File::open(file).unwrap();
    // let lines = io::BufReader::new(file).lines().map(|x| x.unwrap().chars()).collect();
    let mut data = fs::read_to_string(file).unwrap();
    data = data.replace("\n", "");
    return data;
}

fn part_one(data: String) {
    let p1 = data.chars();
    let p2 = data.chars().skip(1);
    let p3 = data.chars().skip(2);
    let p4 = data.chars().skip(3);
}

#[cfg(test)]
mod tests {
    use super::WordGrid;

    #[test]
    fn test_check_xmas1() {
        let grid = WordGrid::new("day4test.txt");
        assert_eq!(3, grid.find_all_horizontal())
    }

    #[test]
    fn test_check_xmas2() {
        let grid = WordGrid::new("day4test.txt");
        assert_eq!(1, grid.find_all_vertical())
    }

    #[test]
    fn test_check_xmas3() {
        let grid = WordGrid::new("day4test.txt");
        assert_eq!(18, grid.find_all())
    }
}
