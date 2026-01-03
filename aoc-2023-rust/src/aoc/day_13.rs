use crate::aoc::grid::Grid;

struct Puzzle {
    maps: Vec<Grid<char>>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let data = data.trim().replace("\r", "");
        let maps: Option<_> = data.split("\n\n")
            .map(|s| s.parse().ok())
            .collect();
        let maps = maps?;
        Some(Self {maps})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> Option<u32> {
    let summary = puzzle.maps.iter()
        .map(|m| {
            let (w, h) = m.size();
            let rs = Vec::from_iter((0..h).map(|i| Vec::from_iter(m.row(i))));
            let cs = Vec::from_iter((0..w).map(|i| Vec::from_iter(m.column(i))));
            if let Some(ri) = (1..rs.len())
                .find(|ri| {
                    let s = usize::min(*ri, h - ri);
                    (0..s).all(|i| rs[ri - i - 1] == rs[ri + i])
                }) {
                (ri * 100) as u32
            } else if let Some(ci) = (1..cs.len())
                .find(|ci| {
                    let s = usize::min(*ci, w - ci);
                    (0..s).all(|i| cs[ci - i - 1] == cs[ci + i])
                }) {
                ci as u32
            } else {
                0
            }
        })
        .sum();
    if summary > 0 {Some(summary)} else {None}
}

fn part_2(puzzle: &Puzzle) -> Option<u32> {
    todo!("part 2");
}

fn solve_part_1(puzzle: &Puzzle) -> u32 {
    part_1(puzzle).expect("valid puzzle")
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    part_2(puzzle).expect("valid puzzle")
}

pub fn solve() {
    let data = include_str!("../../data/day_13/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    #.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.

    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    ";

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_1(&puzzle) == 405);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_2(&puzzle) == 400);
    }
}
