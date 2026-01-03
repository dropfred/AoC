use crate::aoc::grid::Grid;

struct Puzzle {
    map: Grid<char>
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

fn part_1(puzzle: &Puzzle) -> Option<u32> {
    let map = &puzzle.map;
    let h = map.size().1 as u32;
    let s = map.columns()
        .map(|c|
            c.enumerate().fold((0, 0), |(w, s), (i, c)| {
                let i = i as u32;
                match c {
                    'O' => (w + (h - s), s + 1),
                    '#' => (w, i + 1),
                    _   => (w, s)
                }
            }).0
        )
        .sum();
    Some(s)
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
    let data = "
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    ";
    let data = include_str!("../../data/day_14/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#....
    ";

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_1(&puzzle) == 136);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_2(&puzzle) == 64);
    }
}
