use std::{
    fs::{self, read_to_string, File},
    io::{self, BufRead},
};

struct WordGrid {
    data: String,
    width: usize,
    bytedata: Vec<u8>,
    target: Vec<u8>,
    target2: Vec<u8>,
    target3: Vec<u8>,
}

impl WordGrid {
    pub fn new(file: &str) -> Self {
        let data = fs::read_to_string(file).unwrap();
        let width = data.chars().take_while(|&c| c != '\n').count() + 1;
        let bytedata = data.clone().into_bytes();
        let target = String::from("XMAS").into_bytes();
        let target2 = String::from("MSAMS").into_bytes();
        let target3 = String::from("MMASS").into_bytes();

        Self {
            data,
            width,
            bytedata,
            target,
            target2,
            target3,
        }
    }

    fn find_all(&self) -> usize {
        let mut count = 0;
        for i in 0..self.data.len() {
            if self.check_horizontal(i) {
                count += 1
            }
            if self.check_horizontalb(i) {
                count += 1
            }
            if self.check_vertical(i) {
                count += 1
            }
            if self.check_verticalb(i) {
                count += 1
            }
            if self.check_diagonal_1(i) {
                count += 1
            }
            if self.check_diagonal_1b(i) {
                count += 1
            }
            if self.check_diagonal_2(i) {
                count += 1
            }
            if self.check_diagonal_2b(i) {
                count += 1
            }
        }
        return count;
    }

    fn find_all2(self) -> usize {
        let mut count = 0;
        for i in 0..self.data.len() {
            if self.check_x(i) {
                count += 1
            }
            if self.check_xr(i) {
                count += 1
            }
            if self.check_x1(i) {
                count += 1
            }
            if self.check_xr1(i) {
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
        let target = self.target.iter();
        let test = self.bytedata.iter().skip(idx).step_by(1).take(4);
        target.eq(test)
    }

    fn check_horizontalb(&self, idx: usize) -> bool {
        let target = self.target.iter();
        let test = self.bytedata.iter().rev().skip(idx).step_by(1).take(4);

        target.eq(test)
    }

    fn check_vertical(&self, idx: usize) -> bool {
        let target = self.target.iter();

        let test = self.bytedata.iter().skip(idx).step_by(self.width).take(4);
        target.eq(test)
    }

    fn check_verticalb(&self, idx: usize) -> bool {
        let target = self.target.iter();

        let test = self
            .bytedata
            .iter()
            .rev()
            .skip(idx)
            .step_by(self.width)
            .take(4);

        target.eq(test)
    }

    fn check_diagonal_1(&self, idx: usize) -> bool {
        let target = self.target.iter();

        let test = self
            .bytedata
            .iter()
            .skip(idx)
            .step_by(self.width - 1)
            .take(4);

        target.eq(test)
    }

    fn check_diagonal_1b(&self, idx: usize) -> bool {
        let target = self.target.iter();

        let test = self
            .bytedata
            .iter()
            .rev()
            .skip(idx)
            .step_by(self.width - 1)
            .take(4);

        target.eq(test)
    }

    fn check_diagonal_2(&self, idx: usize) -> bool {
        let target = self.target.iter();

        let test = self
            .bytedata
            .iter()
            .skip(idx)
            .step_by(self.width + 1)
            .take(4);

        target.eq(test)
    }

    fn check_diagonal_2b(&self, idx: usize) -> bool {
        let target = self.target.iter();

        let test = self
            .bytedata
            .iter()
            .rev()
            .skip(idx)
            .step_by(self.width + 1)
            .take(4);

        target.eq(test)
    }

    fn check_x(&self, idx: usize) -> bool {
        let offsets = [0, 2, self.width + 1, 2 * (self.width), 2 * (self.width) + 2];

        offsets
            .iter()
            .enumerate()
            .take_while(|(jdx, &offset)| self.bytedata.get(idx + offset) == self.target2.get(*jdx))
            .count()
            == 5
    }

    fn check_xr(&self, idx: usize) -> bool {
        let offsets = [0, 2, self.width + 1, 2 * (self.width), 2 * (self.width) + 2];

        offsets
            .iter()
            .enumerate()
            .take_while(|(jdx, &offset)| {
                if let Some(k) = self.bytedata.len().checked_sub(idx + offset) {
                    self.bytedata.get(k) == self.target2.get(*jdx)
                } else {
                    false
                }
            })
            .count()
            == 5
    }

    fn check_x1(&self, idx: usize) -> bool {
        let offsets = [0, 2, self.width + 1, 2 * (self.width), 2 * (self.width) + 2];

        offsets
            .iter()
            .enumerate()
            .take_while(|(jdx, &offset)| self.bytedata.get(idx + offset) == self.target3.get(*jdx))
            .count()
            == 5
    }

    fn check_xr1(&self, idx: usize) -> bool {
        let offsets = [0, 2, self.width + 1, 2 * (self.width), 2 * (self.width) + 2];

        offsets
            .iter()
            .enumerate()
            .take_while(|(jdx, &offset)| {
                if let Some(k) = self.bytedata.len().checked_sub(idx + offset) {
                    self.bytedata.get(k) == self.target3.get(*jdx)
                } else {
                    false
                }
            })
            .count()
            == 5
    }
}

pub fn solution() {
    let grid = WordGrid::new("day4.txt");
    println!("day 4 part 1: {}", grid.find_all());
    println!("day 4 part 1: {}", grid.find_all2());
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

    #[test]
    fn test_check_xmas4() {
        let grid = WordGrid::new("day4.txt");
        assert_eq!(2297, grid.find_all())
    }

    // #[test]
    // fn test_check_xmas5() {
    //     let grid = WordGrid::new("day4test.txt");
    //     assert_eq!(9, grid.find_all2())
    // }

    #[test]
    fn test_check_xmas6() {
        let grid = WordGrid::new("day4test2.txt");
        assert_eq!(1, grid.find_all2())
    }
}
