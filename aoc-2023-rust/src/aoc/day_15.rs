struct Puzzle {
    steps: Vec<Vec<u8>>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        if !data.is_ascii() {
            return None;
        }
        let data = data.replace(&['\n', '\r', ' '][..], "");
        let steps = data
            .lines()
            .flat_map(|s| s.split(',').map(|s| Vec::from(s.as_bytes())))
            .collect();
        Some(Self {steps})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn hash(ascii: &[u8]) -> u32 {
    let mut h = 0;
    for b in ascii {
        h += *b as u32;
        h *= 17;
        h &= 0xff;
    }
    h
}

fn solve_part_1(puzzle: &Puzzle) -> u32 {
    puzzle.steps.iter().map(|s| hash(s)).sum()
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    todo!("part 2");
}

pub fn solve() {
    let data = include_str!("../../data/day_15/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    rn=1,cm-,
    qp=3,cm=2,
    qp-,pc=4,
    ot=9,ab=5,
    pc-,pc=6,
    ot=7
    ";

    #[test]
    fn test_part_hash() {
        assert!(hash("HASH".as_bytes()) == 52);
    }

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_1(&puzzle) == 1320);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_2(&puzzle) == 0);
    }
}
