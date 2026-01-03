use crate::aoc::grid::Grid;

struct Puzzle {
    plan: Grid<char>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let plan = data.parse().ok()?;
        Some(Self {plan})
    }
 
    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> Option<u32> {
    let plan = &puzzle.plan;
    let (w, h) = plan.size();
    let (iw, ih) = (w as i32, h as i32);
    let is_digit = |x, y| {
        if (x < 0) || (x >= iw) || (y < 0) || (y >= ih) {
            return false;
        }
        let c = plan.get((x as usize, y as usize));
        c.is_ascii_digit()
    };
    let is_symbol = |x, y| {
        if (x < 0) || (x >= iw) || (y < 0) || (y >= ih) {
            return false;
        }
        let c = plan.get((x as usize, y as usize));
        !((c == '.') || c.is_ascii_digit())
    };
    let mut s = 0;
    let mut x = 0;
    let mut y = 0;
    while y < ih {
        while x < iw {
            if is_digit(x, y) {
                let mut n = String::new();
                n.push(plan.get((x as usize, y as usize)));
                let x0 = x;
                loop {
                    x += 1;
                    if is_digit(x, y) {
                        n.push(plan.get((x as usize, y as usize)));
                    } else {
                        break;
                    }
                }
                if is_symbol(x0 - 1, y) ||
                    is_symbol(x, y) ||
                    (x0..x).any(|x| is_symbol(x, y - 1) || is_symbol(x, y + 1)) {
                    let n: u32 = n.parse().unwrap();
                    s += n;
                }
            } else {
                x += 1;
            }

        }
        y += 1;
    }
    Some(s)
}

fn part_2(puzzle: &Puzzle) -> Option<u32> {
    None
}

fn solve_part_1(puzzle: &Puzzle) -> u32 {
    part_1(puzzle).expect("valid puzzle")
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    part_2(puzzle).expect("valid puzzle")
}

pub fn solve() {
    let data = include_str!("../../data/day_03/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_1(&puzzle) == 4361);
    }
        
    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_2(&puzzle) == 2286);
    }
}
