use regex::Regex;
use std::fs;
/// notes. it costs 3 to push A , 1 to push B.
/// find cheapest way to get to prize.
/// each button can only be pressed 100 times.
/// there is no case where a and b are the same.
///
/// approaches.
/// part 1.
/// could go for a depth first search?
/// could do some vector maths?
/// i think there is a unique solution since a and b are always
/// different
///
/// part 2.
/// linear alegbra

#[derive(Debug, PartialEq, Clone, Copy)]
struct ClawMachine {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

// this is part 1
fn load_data(path: &str) -> Vec<ClawMachine> {
    let string = fs::read_to_string(path).unwrap();
    let regex = Regex::new(
        r"(?m)Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let result = regex.captures_iter(&string).map(|c| c.extract());

    let mut claw_machines = vec![];

    for (_, [ax, ay, bx, by, px, py]) in result {
        claw_machines.push(ClawMachine {
            a: (ax.parse().unwrap(), ay.parse().unwrap()),
            b: (bx.parse().unwrap(), by.parse().unwrap()),
            prize: (px.parse().unwrap(), py.parse().unwrap()),
        })
    }
    claw_machines
}

fn load_data2(path: &str) -> Vec<ClawMachine> {
    let string = fs::read_to_string(path).unwrap();
    let regex = Regex::new(
        r"(?m)Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let result = regex.captures_iter(&string).map(|c| c.extract());

    let mut claw_machines = vec![];

    for (_, [ax, ay, bx, by, px, py]) in result {
        claw_machines.push(ClawMachine {
            a: (ax.parse().unwrap(), ay.parse().unwrap()),
            b: (bx.parse().unwrap(), by.parse().unwrap()),
            prize: (
                px.parse::<f64>().unwrap() + 10000000000000.0,
                py.parse::<f64>().unwrap() + 10000000000000.0,
            ),
        })
    }
    claw_machines
}

// Initial approach...
// #[cached]
// fn claw_search(claw_machine: ClawMachine, coord: (usize, usize), a: usize,b: usize) -> Option<(usize, usize)> {
//     if claw_machine.prize == coord {
//         return Some((a,b))
//     } else if coord.0 > claw_machine.prize.0 || coord.1 > claw_machine.prize.1 {
//         return None;
//     } else if a > 100 || b > 100 {
//         return None;
//     }else {
//         // go a
//         let ca = (coord.0 + claw_machine.a.0, coord.1 + claw_machine.a.1);
//         let cb = (coord.0 + claw_machine.b.0, coord.1 + claw_machine.b.1);
//         if let Some(res_a) = claw_search(claw_machine, ca, a+1, b) {
//             return Some(res_a);
//         } else if let Some(res_b) = claw_search(claw_machine, cb, a, b+1) {
//             return Some(res_b);
//         } else {
//             return None;
//         }

//     }
// }

// its been a while, but I eventually remembered how do maths.
fn find_coords(claw_machine: ClawMachine) -> Option<(f64, f64)> {
    let determinant = (claw_machine.a.0 * claw_machine.b.1 - claw_machine.b.0 * claw_machine.a.1);
    let n = (claw_machine.b.1 * claw_machine.prize.0 - claw_machine.b.0 * claw_machine.prize.1)
        / determinant;
    let m = (claw_machine.a.0 * claw_machine.prize.1 - claw_machine.a.1 * claw_machine.prize.0)
        / determinant;

    if n.round() == n && m.round() == m {
        Some((n, m))
    } else {
        None
    }
}

pub fn solution() {
    let claw_machines = load_data("day13.txt");

    let mut total = 0.0;
    for m in claw_machines {
        // recursive solution will stack overflow for part 2
        let res = find_coords(m);
        if let Some(res) = res {
            total += res.0 * 3.0 + res.1;
        }
    }

    println!("Total part 1={total}");

    let claw_machines = load_data2("day13.txt");

    let mut total = 0.0;
    for m in claw_machines {
        // recursive solution will stack overflow for part 2
        let res = find_coords(m);
        if let Some(res) = res {
            total += res.0 * 3.0 + res.1;
        }
    }

    println!("Total part 2 ={total}")
}
