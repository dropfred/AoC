struct Race {
    time: u64,
    distance: u64
}
struct Puzzle {
    races: Vec<Race>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let mut ls = data.trim().lines();
        let ts = ls.next()?.trim().strip_prefix("Time:")?.split_whitespace();
        let ds = ls.next()?.trim().strip_prefix("Distance:")?.split_whitespace();
        if ls.next().is_some() {
            return None;
        }
        let parse_race = |td: (&str, &str)| Some(Race {time: td.0.parse().ok()?, distance: td.1.parse().ok()?});
        let races: Option<_> = ts.zip(ds).map(|td| parse_race(td)).collect();
        let races = races?;
        Some(Self {races})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn get_num_wins(r: &Race) -> u64 {
    // assumes that the record can always be beaten
    let d = ((r.time.pow(2) - 4 * r.distance) as f64).sqrt();
    let mut t1 = (r.time as f64 - d) / 2.0;
    let mut t2 = (r.time as f64 + d) / 2.0;
    if t1.fract() == 0.0 {t1 += 1.0;}
    if t2.fract() == 0.0 {t2 -= 1.0;}
    let t1 = t1.ceil() as u64;
    let t2 = t2.floor() as u64;
    (t2 - t1) + 1
}

fn part_1(puzzle: &Puzzle) -> u64 {
    puzzle.races.iter().map(get_num_wins).product()
}

fn part_2(puzzle: &Puzzle) -> u64 {
    let (time, distance) = puzzle.races.iter()
        .fold((0, 0), | (t, d), r| {
            let (rt, rd) = (r.time as u64, r.distance as u64);
            (
                (t * 10u64.pow(r.time.ilog10() + 1)) + rt,
                (d * 10u64.pow(r.distance.ilog10() + 1)) + rd
            )
        });
    get_num_wins(&Race {time, distance})
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_06/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    Time:      7  15   30
    Distance:  9  40  200
    ";

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(part_1(&puzzle), 288);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(part_2(&puzzle), 71503);
    }
}
