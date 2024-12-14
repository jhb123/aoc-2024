// it aint pretty, but it works
use std::{collections::HashSet, fs};

use log::{debug, info};

pub fn solution() {
    part1();
    let data_i = fs::read_to_string("day9.txt").unwrap();
    // let data_i = "2333133121414131402";
    //let data_i = "13201";
    part2(&data_i);
}

#[derive(Debug)]
struct Move {
    space_loc: usize,
    file_id: usize,
    file_size: usize,
}
impl Move {
    fn new(space_loc: usize, file_id: usize, file_size: usize) -> Self {
        Self {
            space_loc,
            file_id,
            file_size,
        }
    }
}

fn part2(data_i: &str) -> usize {
    // let data_i = "2333133121414131402";
    //let data_i = fs::read_to_string("day9.txt").unwrap();

    let mut data: Vec<usize> = data_i
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    data.push(0);
    let (even, odd): (Vec<_>, Vec<_>) = data.into_iter().enumerate().partition(|(i, _)| i % 2 == 0);

    let file_sizes: Vec<usize> = even.iter().map(|x| x.1).collect();
    let mut space_sizes: Vec<usize> = odd.iter().map(|x| x.1).collect();

    let mut uncompressed = Vec::new();
    for (i, (a, b)) in file_sizes.iter().zip(space_sizes.iter()).enumerate() {
        uncompressed.append(&mut vec![i; *a]);
        uncompressed.append(&mut vec![0; *b]);
    }

    let mut moves = Vec::<Move>::new();

    for (i, file_size) in file_sizes.iter().enumerate().rev() {
        if i == 0 {
            break;
        }
        let mut space = space_sizes.iter_mut().enumerate();
        while let Some(x) = space.next() {
            if x.0 == i {
                break;
            }
            if *x.1 < *file_size {
                continue;
            } else {
                moves.push(Move::new(x.0, i, *file_size));
                *x.1 = *x.1 - *file_size;
                break;
            }
        }
    }
    // println!("Moves {:?}", moves);
    // println!("space_sizes {:?}", space_sizes);
    // println!("file sizes {:?}", file_sizes);
    debug!("Uncompressed {:?}", uncompressed);
    let mut compressed = Vec::new();
    let mut swapped = HashSet::new();
    debug!("Moves {:?}", moves);

    for (file_space_i, (file_size, space_size)) in
        file_sizes.iter().zip(space_sizes.iter()).enumerate()
    {
        debug!(
            "start iter. file id: {file_space_i}, file size: {file_size}, num spaces {space_size}"
        );
        if let Some(_x) = moves
            .iter()
            .map(|m| m.file_id)
            .find(|file_id| *file_id == file_space_i)
        {
            if let None = swapped.get(&file_space_i) {
                debug!(
                    "Found in moves, but not swapped yet. Inserting {:?}",
                    vec![file_space_i; *file_size]
                );
                compressed.append(&mut vec![file_space_i; *file_size]);
                swapped.insert(file_space_i);
            } else {
                debug!("Swapped already, so inserting {:?}", vec![0; *file_size]);
                compressed.append(&mut vec![0; *file_size]);
            }
        } else {
            debug!(
                "Not moved ever, so inserting {:?}",
                vec![file_space_i; *file_size]
            );
            compressed.append(&mut vec![file_space_i; *file_size]);
        }
        for m in &moves {
            if m.space_loc == file_space_i {
                if let None = swapped.get(&m.file_id) {
                    debug!(
                        "found move {:?}, inserting {:?}",
                        m,
                        vec![m.file_id; m.file_size]
                    );
                    compressed.append(&mut vec![m.file_id; m.file_size]);
                    swapped.insert(m.file_id);
                } else {
                    debug!(
                        "Swapped already, so inserting {:?} instead of ",
                        vec![0; *file_size]
                    );
                    compressed.append(&mut vec![0; *file_size]);
                }
            }
        }

        compressed.append(&mut vec![0; *space_size]);
        debug!("Inserting space {:?}", vec![0; *space_size]);

        debug!("end iter. file id: {file_space_i}, file size: {file_size}, num spaces {space_size}. compressed {:?}",compressed);
        // compressed.append(&mut vec![i;*a]);
        // uncompressed.append(&mut vec![0;*b]);
    }
    // println!("{:?}",compressed);

    let checksum = compressed
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + i * x);

    // println!("uncompressed = {:?}", uncompressed);
    // println!("compressed = {:?}", compressed);
    info!("checksum = {:?}", checksum);
    return checksum;
    // println!("File sizes = {:?}\nSpace sizes = {:?}\nuncompressed={:?}",file_sizes,space_sizes,uncompressed);
}

fn part1() {
    //let data_i = "2333133121414131402";
    let data_i = fs::read_to_string("day9.txt").unwrap();
    let mut data: Vec<usize> = data_i
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    data.push(0); // last data doesn't need any padding.
    let required_space = data.iter().fold(0, |acc: usize, x| acc + x);
    // println!("Required space: {}", required_space);

    let zeros = data
        .iter()
        .skip(1)
        .step_by(2)
        .fold(0, |acc: usize, x| acc + x);
    // println!("Required space: {}, Zeros: {}", required_space, zeros);

    let mut uncompressed = Vec::with_capacity(required_space);
    let mut compressed = Vec::with_capacity(required_space - zeros);

    for (i, x) in data.chunks(2).enumerate() {
        uncompressed.append(&mut vec![i + 1; x[0]]); // add 1 so that 0 represents empty
        uncompressed.append(&mut vec![0; x[1]]);
    }

    let mut forward_iter = uncompressed.iter().enumerate();
    let mut backward_iter = uncompressed.iter().rev();

    while let Some((i, &a)) = forward_iter.next() {
        if i == required_space - zeros {
            break;
        }
        if a != 0 {
            compressed.push(a);
            continue;
        }
        while let Some(&b) = backward_iter.next() {
            if b == 0 {
                continue;
            }
            compressed.push(b);
            break;
        }
    }
    compressed.iter_mut().for_each(|x| *x -= 1);

    let checksum = compressed
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + i * x);

    // println!("{:?}", data);
    // println!("{:?}", uncompressed);
    // println!("{:?}", compressed);
    println!("{:?}", checksum);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::part2;

    #[test]
    fn test_ans() {
        let data_i = fs::read_to_string("day9.txt").unwrap();
        assert_eq!(part2(&data_i), 6412390114238)
    }

    #[test]
    fn test_example() {
        assert_eq!(part2("2333133121414131402"), 2858)
    }

    #[test]
    fn test_1() {
        assert_eq!(part2("1"), 0)
    }
    #[test]
    fn test_2() {
        assert_eq!(part2("2"), 0)
    }
    #[test]
    fn test_3() {
        assert_eq!(part2("122"), 3)
    }
    #[test]
    fn test_4() {
        assert_eq!(part2("123"), 0 + 0 + 0 + 3 + 4 + 5)
    }

    #[test]
    fn test_5() {
        assert_eq!(part2("12302"), 0 + 2 + 4 + 3 + 4 + 5)
    }
    #[test]
    fn test_6() {
        assert_eq!(part2("12344"), 0 + 0 + 0 + 3 + 4 + 5 + 2 * (6 + 7 + 8 + 9))
    }
    #[test]
    fn test_7() {
        assert_eq!(part2("12202"), 0 + 2 * (1 + 2) + (3 + 4))
    }

    #[test]
    fn test_8() {
        assert_eq!(part2("11111"), 0 + 2 + 2)
    }
}
