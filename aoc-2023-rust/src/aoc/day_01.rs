struct Puzzle {
    calibration: Vec<String>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let calibration = data.trim().lines().map(|c| c.trim().to_string()).collect();
        Some(Self {calibration})
    }
 
    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> Option<u32> {
    let parse_calibration = |c: &String| {
        let t = c.chars().find(|c| c.is_ascii_digit())?.to_digit(10)?;
        let u = c.chars().rfind(|c| c.is_ascii_digit())?.to_digit(10)?;
        Some(t * 10 + u)
    };
    let c: Option<Vec<_>> = puzzle.calibration.iter().map(parse_calibration).collect();
    let s = c?.iter().sum();
    Some(s)
}

fn part_2(puzzle: &Puzzle) -> Option<u32> {
    const DIGITS: [&str; 10] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];
    let mut s = 0;
    for cal in puzzle.calibration.iter() {
        let d = cal.chars().enumerate().find(|(_, c)| c.is_ascii_digit()).map(|(i, c)| (i, c.to_digit(10).unwrap()));
        let a = (0..10).into_iter()
            .map(|i| (cal.find(&DIGITS[i]), i as u32))
            .filter(|(i, _)| i.is_some())
            .map(|(i, v)| (i.unwrap(), v))
            .min_by(|a, b| {a.0.cmp(&b.0)});
        let t = [d, a].into_iter()
            .filter(|n| n.is_some())
            .min_by(|a, b| {a.unwrap().0.cmp(&b.unwrap().0)})
            .unwrap()?;

        let d = cal.chars().rev().enumerate().find(|(_, c)| c.is_ascii_digit()).map(|(i, c)| (cal.len() - i - 1, c.to_digit(10).unwrap()));
        let a = (0..10).into_iter()
            .map(|i| (cal.rfind(&DIGITS[i]), i as u32))
            .filter(|(i, _)| i.is_some())
            .map(|(i, v)| (i.unwrap(), v))
            .max_by(|a, b| {a.0.cmp(&b.0)});
        let u = [d, a].into_iter()
            .filter(|n| n.is_some())
            .max_by(|a, b| {a.unwrap().0.cmp(&b.unwrap().0)})
            .unwrap()?;

        s += t.1 * 10 + u.1;
    }
    Some(s)
}

fn solve_part_1(puzzle: &Puzzle) -> u32 {
    part_1(puzzle).expect("valid puzzle")
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    part_2(puzzle).expect("valid puzzle")
}

pub fn solve() {
    let data = include_str!("../../data/day_01/input.txt");
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
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";
        let puzzle = Puzzle::load(data);
        assert!(solve_part_1(&puzzle) == 142);
    }

    #[test]
    fn test_part_2() {
        let data = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ";
        let puzzle = Puzzle::load(data);
        assert!(solve_part_2(&puzzle) == 281);
    }
}
