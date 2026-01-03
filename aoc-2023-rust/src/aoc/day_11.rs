use crate::aoc::grid::Grid;

struct Puzzle {
    galaxies: Vec<(u64, u64)>,
    empty_rows: Vec<u64>,
    empty_columns: Vec<u64>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let map: Grid<char> = data.parse().ok()?;
        let galaxies = map.cells()
            .filter_map(|((x, y), c)| if c != '.' {Some((x as u64, y as u64))} else {None})
            .collect();
        let empty_rows = (0..(map.size().1))
            .filter_map(|i| {
                if map.row(i).all(|c| c == '.') {Some(i as u64)} else {None}
            })
            .collect();
        let empty_columns = (0..(map.size().0))
            .filter_map(|i| {
                if map.column(i).all(|c| c == '.') {Some(i as u64)} else {None}
            })
            .collect();
        Some(Self {galaxies, empty_rows, empty_columns})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }

    fn solve(&self, factor: u64) -> u64 {
        let space = factor - 1;
        let galaxies = Vec::from_iter(self.galaxies.iter()
            .map(|(x, y)| {
                let (mut ex, mut ey) = (*x as u64, *y as u64);
                self.empty_columns.iter().for_each(|e| {if *x > *e {ex += space;}});
                self.empty_rows   .iter().for_each(|e| {if *y > *e {ey += space;}});
                (ex, ey)
            }));
    
        let mut distance = 0;
        for i in 0..(galaxies.len() - 1) {
            let (x1, y1) = galaxies[i];
            for j in (i + 1)..galaxies.len() {
                let (x2, y2) = galaxies[j];
                distance += u64::abs_diff(x1, x2) + u64::abs_diff(y1, y2);
            }
        }
    
        distance
    }
}

fn solve_part_1(puzzle: &Puzzle) -> u64 {
    puzzle.solve(2)
}

fn solve_part_2(puzzle: &Puzzle) -> u64 {
    puzzle.solve(1000000)
}

pub fn solve() {
    let data = include_str!("../../data/day_11/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let data = "
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        ";
        let puzzle = Puzzle::load(data);
        assert!(puzzle.solve(2) == 374);
        assert!(puzzle.solve(10) == 1030);
        assert!(puzzle.solve(100) == 8410);
    }

}
