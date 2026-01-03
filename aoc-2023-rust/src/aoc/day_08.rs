use std::collections::HashMap;

struct Puzzle {
    navigation: String,
    network: HashMap<String, (String, String)>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let mut data = data.trim().lines();
        let navigation = data.next()?.trim().to_string();
        {
            let e = data.next()?.trim();
            if !e.is_empty() {return None;}
        }
        let parse_node = |s: &str| {
            let (n, lr) = s.trim().split_once(" = ")?;
            let (l, r) = lr.strip_prefix("(")?.strip_suffix(")")?.split_once(", ")?;
            let name = n.to_string();
            let left = l.to_string();
            let right = r.to_string();
            Some((name, (left, right)))
        };
        let network: Option<_> = data.map(parse_node).collect();
        let network = network?;
        Some(Self {navigation, network})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_num_steps(puzzle: &Puzzle, start: &str) -> Option<u64> {
    let mut navigation = puzzle.navigation.chars().cycle();
    let mut steps = 0;
    let mut n = start;
    // assume no loop in the navigation before reaching Z
    loop {
        if n.ends_with('Z') {
            break;
        }
        steps += 1;
        let lr = puzzle.network.get(n)?;
        let d = navigation.next()?;
        n = if d == 'L' {&lr.0} else  {&lr.1};
    }
    Some(steps)
}

fn part_1(puzzle: &Puzzle) -> Option<u64> {
    // assume that "AAA" leads to "ZZZ"
    get_num_steps(puzzle, "AAA")
}

fn solve_part_1(puzzle: &Puzzle) -> u64 {
    part_1(puzzle).expect("solvable puzzle")
}

fn gcd(a: u64, b: u64) -> u64 {
    let (mut a, mut b) = (a, b);
    if a != b {
        if b > a {
            (a, b) = (b, a)
        }
        while b > 0 {
            (a, b) = (b, a % b)
        }
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b
}

fn part_2(puzzle: &Puzzle) -> Option<u64> {
    let ss: Option<Vec<_>> = puzzle.network.iter()
        .filter_map(|(n, _)| {
            if n.ends_with('A') {Some(n)} else {None}
        })
        .map(|n| get_num_steps(puzzle, n))
        .collect();
    let ss = ss?;
    // assume that Z loops to A
    let ss = ss.into_iter().reduce(|a, n| lcm(a, n))?;
    Some(ss)
}

fn solve_part_2(puzzle: &Puzzle) -> u64 {
    part_2(puzzle).expect("solvable puzzle")
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_08/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "
        RL
    
        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
        ";
        let puzzle = Puzzle::load(data);
        assert_eq!(solve_part_1(&puzzle), 2);

        let data = "
        LLR
    
        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        ";
        let puzzle = Puzzle::load(data);
        assert_eq!(solve_part_1(&puzzle), 6);
    }

    #[test]
    fn test_part_2() {
        let data = "
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        ";
        let puzzle = Puzzle::load(data);
        assert_eq!(solve_part_2(&puzzle), 6);
    }
}
