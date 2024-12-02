use std::{collections::HashMap, fs::File, io::{self, BufRead}, iter::zip};

pub fn day_one_part_one(mut list_a: Vec<i64>, mut list_b: Vec<i64>) -> i64 {
    list_a.sort();
    list_b.sort();

    let iter = zip(list_a, list_b);

    iter.fold(0,|acc,b| {
        acc + (b.0 - b.1).abs()
    })

}

pub  fn day_one_part_two(list_a: Vec<i64>, list_b: Vec<i64>) -> i64 {

    let map = list_b.iter().fold(HashMap::new(), |mut map: HashMap<i64, i64> , &k | {
        match map.get(&k) {
            Some(v) => map.insert(k, v+1),
            None => map.insert(k, 1),
        };
        map
    } );

    list_a.iter().fold(0, |acc, k| {
        if let Some(x) = map.get(k) {
            acc + k*x
        } else {
            acc
        }
    } )

}

pub fn load_day_one() -> (Vec<i64>,Vec<i64>) {
    let file = File::open("day1.txt").expect("missing the test data");
    let lines: Vec::<String> = io::BufReader::new(file).lines().collect::<Result<_,_>>().unwrap(); 
    
    let mut list_a: Vec<i64> = Vec::with_capacity(lines.len());
    let mut list_b: Vec<i64>  = Vec::with_capacity(lines.len());

    for line in lines {
        let mut nums = line.split("   ");
        list_a.push(nums.next().unwrap().parse().unwrap());
        list_b.push(nums.next().unwrap().parse().unwrap());
    }

    return (list_a, list_b)
}


#[cfg(test)]
mod tests {
    use crate::{day_one_part_one, day_one_part_two, load_day_one};

    #[test]
    fn test_day_one_part_1(){
        let list_a = vec![3,4,2,1,3,3];
        let list_b = vec![4,3,5,3,9,3];

        assert_eq!(11, day_one_part_one(list_a, list_b))
    }

    #[test]
    fn test_day_one_part_2(){
        let list_a = vec![3,4,2,1,3,3];
        let list_b = vec![4,3,5,3,9,3];

        assert_eq!(31, day_one_part_two(list_a, list_b))
    }

    #[test]
    fn test_load_day_one(){
        let (list_a, list_b) = load_day_one();
        assert_eq!(77442, list_a[0]);
        assert_eq!(29363, list_a[list_a.len()-1]);
        assert_eq!(88154, list_b[0]);
        assert_eq!(98020, list_b[list_b.len()-1]);
    }

}