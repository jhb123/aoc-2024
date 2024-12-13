use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

pub fn solution() {
    let (pages, rules) = load_data("day5.txt");
    solve_part_one(&pages, &rules);

    solve_part_two(pages, &rules);
}

type Pages = Vec<i64>;
type Rules = HashMap<i64, HashSet<i64>>;

struct PageOrderer {
    pages: Pages,
    rules: Rules,
}

#[derive(PartialEq)]
enum PagesState {
    Forbidden,
    Allowed,
}

impl PageOrderer {
    fn new(pages: Pages, rules: Rules) -> Self {
        Self { pages, rules }
    }

    fn is_valid_backwards(&self) -> bool {
        // check forward if it is forbidden.
        let mut forbidden: HashSet<i64> = HashSet::new();
        let valid_forward = self
            .pages
            .iter()
            .rev()
            .map(|x| {
                if forbidden.contains(x) {
                    PagesState::Forbidden
                } else {
                    if let Some(f) = self.rules.get(x) {
                        f.iter().for_each(|x| {
                            forbidden.insert(*x);
                        }); // forbidden.union(&f).collect();
                    }
                    PagesState::Allowed
                }
            })
            .collect::<Vec<PagesState>>()
            .iter()
            .all(|x| *x == PagesState::Allowed);

        // check backward to make sure it satifies the rules

        return valid_forward;
    }

    fn is_valid_forwards(&self) -> bool {
        // check forward if it is forbidden.
        let mut required: HashSet<i64> = HashSet::new();
        let _valid_forward = self.pages.iter().for_each(|x| {
            required.remove(x);
            if let Some(f) = self.rules.get(x) {
                f.iter().for_each(|x| {
                    required.insert(*x);
                });
            }
            // println!("x={}, required: {:?}",x, required);
        });

        if required.is_empty() {
            return true;
        } else {
            for x in required {
                if self.pages.contains(&x) {
                    return false;
                }
            }
            return true;
        }
    }

    fn is_valid(&self) -> bool {
        return self.is_valid_backwards() && self.is_valid_forwards();
    }
}

fn rule_subset(pages: &Pages, rules: &Rules) -> Rules {
    //
    let mut rule_subset = HashMap::new();
    for k in pages.iter() {
        rules.get(k).and_then(|rule| {
            rule_subset.insert(*k, rule.clone());
            Some(rule)
        });
    }
    rule_subset
}

fn root(rules: &Rules) -> i64 {
    // I could probably make some sort of better design
    // which doesn't require rule subset to be called before
    // this method is called.
    let lhs: HashSet<i64> = rules.keys().cloned().collect();
    let mut rhs = HashSet::new();
    rules.iter().for_each(|(_, x)| {
        x.iter().for_each(|&num| {
            rhs.insert(num);
        });
    });
    let foo: HashSet<_> = lhs.difference(&rhs).collect();
    let root = foo.iter().next().unwrap();
    return **root;
}

fn load_data(data: &str) -> (Vec<Pages>, Rules) {
    // println!("loading: {data}");
    let f = File::open(data).unwrap();
    let lines = io::BufReader::new(&f).lines();
    let mut rules: Rules = HashMap::new();
    lines
        .take_while(|l| !l.as_ref().unwrap().is_empty())
        .for_each(|l| {
            let l = l.unwrap();
            let nums: Vec<i64> = l.split("|").map(|c| c.parse().unwrap()).collect();
            // map.insert(k, v)
            if rules.contains_key(&nums[0]) {
                rules.get_mut(&nums[0]).unwrap().insert(nums[1]);
            } else {
                let mut s = HashSet::new();
                s.insert(nums[1]);
                rules.insert(nums[0], s);
            }
        });

    let f = File::open(data).unwrap();
    let lines = io::BufReader::new(&f).lines();
    let pages: Vec<Pages> = lines
        .skip_while(|l| !l.as_ref().unwrap().is_empty())
        .skip(1)
        .map(|s| {
            s.unwrap()
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let _sum = pages.iter().fold(0, |acc, page| {
        let page_order = PageOrderer::new(page.to_vec(), rules.clone());
        if page_order.is_valid() {
            acc + *page.get(page.len() / 2).unwrap()
        } else {
            acc
        }
    });

    return (pages, rules);
}

fn solve_part_one(pages: &Vec<Pages>, rules: &Rules) {
    let sum = pages.iter().fold(0, |acc, page| {
        let page_order = PageOrderer::new(page.to_vec(), rules.clone());
        if page_order.is_valid() {
            acc + *page.get(page.len() / 2).unwrap()
        } else {
            acc
        }
    });

    println!("day 5 part 1: {sum}")
}

fn order_pages(page: Pages, rules: &Rules) -> Pages {
    // Get all the pages. generate a set of rules which
    // is relevant to the pages you have. Check in the
    // rules for a page which has no which has no rules
    // about what  it must be after. this can placed freely
    // at the start. Remove this page from consideration
    // and repeat the process until you either have no
    // more rules or now more pages to process.

    let mut soln = vec![];
    let mut sub_pages: Pages = page.clone();
    while !sub_pages.is_empty() {
        let rules_subset = rule_subset(&sub_pages, rules);
        if rules_subset.is_empty() {
            soln.append(&mut sub_pages);
            break;
        }
        let root = root(&rules_subset);
        soln.push(root);
        if let Some(idx) = sub_pages.iter().position(|x| *x == root) {
            sub_pages.remove(idx);
        }
    }
    soln
}

fn solve_part_two(pages: Vec<Pages>, rules: &Rules) {
    let sum = pages.iter().fold(0, |acc, page| {
        let page_order = PageOrderer::new(page.to_vec(), rules.clone());
        if !page_order.is_valid() {
            let ordered_page = order_pages(page.to_vec(), rules);
            acc + *ordered_page.get(ordered_page.len() / 2).unwrap()
        } else {
            acc
        }
    });
    println!("day 5 part 2: {sum}")
}
