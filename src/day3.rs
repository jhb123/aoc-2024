use std::fs;
use regex::Regex;

pub fn solution(){
    let s: String = fs::read_to_string("day3.txt").unwrap();
    println!("day 3, part 1: {}",evalutate(&s));
    println!("day 3, part 2: {}",evalutate2(&s));

}


fn evalutate(s: &str) -> usize {
    let re = Regex::new(r"(?m)mul\(\d+,\d+\)").unwrap();
    let re2 = Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap();
    let mut res = 0;
    for capture in re.captures_iter(s){
        let s = &capture[0];
        let caps = re2.captures(s).unwrap();
        res += caps[1].parse::<usize>().unwrap() * caps[2].parse::<usize>().unwrap();
    }

    return res
}

fn evalutate2(s: &str) -> usize {
    // find all naturally accuring dints -> dxnts
    // find all donts -> dints
    // prepend with do, append with dint ?
    // split at dints.
    // find the next do.

    let regex = Regex::new(r"(?m)din't").unwrap();
    let substitution = "dxn't";
    let result = regex.replace_all(s, substitution);


    let regex = Regex::new(r"(?m)don't").unwrap();
    let substitution = "din't";
    let result = regex.replace_all(&result, substitution);

    let prepared_s = format!("do{}din't",&result);

    prepared_s.split("din't").fold(0, |acc, s: &str| {
            acc + s.split("do").skip(1).fold(0, |acc2, s2| acc2 + evalutate(s2))
        }
    )
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_example(){
        let s = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161,evalutate(s));
    }

    #[test]
    fn test_example2(){
        let s = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48,evalutate2(s));
    }

}