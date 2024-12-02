use aoc::{day_one_part_one, day_one_part_two, load_day_one};

fn main() {
    solve_day_one();
}


fn solve_day_one() {

    let (list_a, list_b) = load_day_one();
    let solution1 = day_one_part_one(list_a, list_b);

    let (list_a, list_b) = load_day_one();
    let solution2 = day_one_part_two(list_a, list_b);

    println!("Day 1 part 1: {solution1}");
    println!("Day 1 part 2: {solution2}");

}