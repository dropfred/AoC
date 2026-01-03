use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::aoc::grid::Grid;

struct Puzzle {
    map: Grid<u8>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let map = data.parse().ok()?;
        Some(Self {map})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn solve_part_1(puzzle: &Puzzle) -> u32 {
    let (w, h) = puzzle.map.size();
    let mut pps:Grid<(usize, usize)> = Grid::new(puzzle.map.size(), (0, 0));
    for (p, pp, d) in puzzle.map.explore_weighted((0, 0), |p, _, d| {
        let w = puzzle.map.get(p) as usize;
        Some(d + w)
    }) {
        // println!("{:?}", p);
        // println!("{:?} + {} -> {:?}", pp, d, p);
        pps.set(p, pp);
        if p == (w - 1, h - 1) {
            break;
        }
    }
    let mut path = Vec::new();
    let mut p = (w - 1, h - 1);
    loop {
        path.push(p);
        if p == (0, 0) {
            break;
        }
        p = pps.get(p);
    }
    path.reverse();
    println!("{:?}", path);

    0
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    todo!("part 1");
}

pub fn solve() {
    let data = "
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    ";
    // let data = include_str!("../../data/day_17/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    ";

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_1(&puzzle) == 102);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_2(&puzzle) == 0);
    }
}
