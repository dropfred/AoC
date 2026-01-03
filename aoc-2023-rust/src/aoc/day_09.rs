struct Puzzle {
    report: Vec<Vec<i32>>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_sensor = |s: &str| {
            let vs: Option<_> = s.trim()
                .split_ascii_whitespace()
                .map(|s| s.parse().ok())
                .collect();
            let vs = vs?;
            Some(vs)
        };
        let report: Option<_> = data.trim().lines().map(parse_sensor).collect();
        let report = report?;
        Some (Self {report})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> Option<i64> {
    let mut snv = 0;
    for vs in puzzle.report.iter() {
        let mut nv = 0;
        let mut vs = vs.clone();
        for i in (1..vs.len()).rev() {
            if (0..=i).all(|i| vs[i] == 0) {break;}
            nv += vs[i] as i64;
            for i in 0..i {
                vs[i] = vs[i + 1] - vs[i];
            }
        }
        snv += nv;
    }
    Some(snv)
}

fn solve_part_1(puzzle: &Puzzle) -> i64 {
    part_1(puzzle).expect("solvable puzzle")
}

fn part_2(puzzle: &Puzzle) -> Option<i64> {
    let mut snv = 0;
    for vs in puzzle.report.iter() {
        let mut nvs = Vec::new();
        let mut vs = vs.clone();
        for i in (1..vs.len()).rev() {
            if (0..=i).all(|i| vs[i] == 0) {break;}
            nvs.push(vs[0]);
            for i in 0..i {
                vs[i] = vs[i + 1] - vs[i];
            }
        }
        nvs.reverse();
        let nv = nvs.into_iter().reduce(|a, v| v - a)?;
        snv += nv as i64;
    }
    Some(snv)
}

fn solve_part_2(puzzle: &Puzzle) -> i64 {
    part_2(puzzle).expect("solvable puzzle")
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_09/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    ";

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(solve_part_1(&puzzle), 114);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(solve_part_2(&puzzle), 2);
    }
}
