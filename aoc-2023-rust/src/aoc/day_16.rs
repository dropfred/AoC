use std::collections::HashSet;
use crate::aoc::grid::Grid;

struct Puzzle {
    contraption: Grid<char>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let contraption = data.parse().ok()?;
        Some(Self {contraption})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_activated_tiles(contraption: &Grid<char>, start: (i32, i32), dir: char) -> u32 {
    let (w, h) = contraption.size();
    let (iw, ih) = (w as i32, h as i32);
    let mut vs = HashSet::new();
    let mut cs = vec![(start, dir)];
    while !cs.is_empty() {
        let (p, d) = cs.pop().unwrap();
        let (x, y) = p;
        if (x < 0) || (x >= iw) || (y < 0) || (y >= ih) {
            continue;
        }
        if vs.contains(&(p, d)) {
            continue;
        }
        vs.insert((p, d));
        let c = contraption.get((x as usize, y as usize));
        match (d, c) {
            ('^', '\\') => cs.push(((x - 1, y), '<')),
            ('v', '\\') => cs.push(((x + 1, y), '>')),
            ('<', '\\') => cs.push(((x, y - 1), '^')),
            ('>', '\\') => cs.push(((x, y + 1), 'v')),
            ('^', '/') => cs.push(((x + 1, y), '>')),
            ('v', '/') => cs.push(((x - 1, y), '<')),
            ('<', '/') => cs.push(((x, y + 1), 'v')),
            ('>', '/') => cs.push(((x, y - 1), '^')),
            ('^', '-') => {cs.push(((x - 1, y), '<')); cs.push(((x + 1, y), '>'));},
            ('v', '-') => {cs.push(((x - 1, y), '<')); cs.push(((x + 1, y), '>'));},
            ('<', '-') => cs.push(((x - 1, y), '<')),
            ('>', '-') => cs.push(((x + 1, y), '>')),
            ('^', '|') => cs.push(((x, y - 1), '^')),
            ('v', '|') => cs.push(((x, y + 1), 'v')),
            ('<', '|') => {cs.push(((x, y - 1), '^')); cs.push(((x, y + 1), 'v'));},
            ('>', '|') => {cs.push(((x, y - 1), '^')); cs.push(((x, y + 1), 'v'));},
            ('^', '.') => cs.push(((x, y - 1), '^')),
            ('v', '.') => cs.push(((x, y + 1), 'v')),
            ('>', '.') => cs.push(((x + 1, y), '>')),
            ('<', '.') => cs.push(((x - 1, y), '<')),
            _ => panic!("unexpected {d} / {c} at {:?}", p)
        }
    }
    let cs: HashSet<_> = vs.into_iter().map(|(p, _)| p).collect();
    cs.len() as u32
}

fn solve_part_1(puzzle: &Puzzle) -> u32 {
    get_activated_tiles(&puzzle.contraption, (0, 0), '>')
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    let (w, h) = puzzle.contraption.size();
    let (iw, ih) = (w as i32, h as i32);
    let vs = (0..iw).flat_map(|i| [
        get_activated_tiles(&puzzle.contraption, (i, ih - 1), '^'),
        get_activated_tiles(&puzzle.contraption, (i, 0), 'v')
    ]);
    let hs = (0..ih).flat_map(|i| [
        get_activated_tiles(&puzzle.contraption, (iw - 1, i), '<'),
        get_activated_tiles(&puzzle.contraption, (0, i), '>')
    ]);
    vs.chain(hs).max().unwrap()
}

pub fn solve() {
    let data = include_str!("../../data/day_16/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = r#"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
    "#;

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_1(&puzzle) == 46);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_2(&puzzle) == 0);
    }
}
