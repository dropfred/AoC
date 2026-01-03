struct  Rgb {
    r: usize,
    g: usize,
    b: usize,
}

impl Rgb {
    fn new(r: usize, g: usize, b: usize) -> Self {
        Self {r, g, b}
    }

    fn zero() -> Self {
        Self {r: 0, g: 0, b: 0}
    }
}

struct Game {
    id: u32,
    sets: Vec<Rgb>
}

struct Puzzle {
    games: Vec<Game>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_set = |s: &str| {
            let mut rgb = Rgb::zero();
            for s in s.split(", ") {
                let (n, c) = s.split_once(" ")?;
                match c {
                    "red"   => rgb.r = n.parse().ok()?,
                    "green" => rgb.g = n.parse().ok()?,
                    "blue"  => rgb.b = n.parse().ok()?,
                    _ => return None
                }
            }
            Some(rgb)
        };
        let parse_game = |s: &str| {
            let s = s.trim();
            let (id, sets) = s.split_once(": ")?;
            let id = id.strip_prefix("Game ")?.parse().ok()?;
            let sets: Option<_> = sets.split("; ").map(parse_set).collect();
            let sets = sets?;
            Some(Game {id, sets})
        };
        let games: Option<_> = data.trim().lines().map(parse_game).collect();
        let games = games?;
        Some(Self {games})
    }
 
    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> Option<u32> {
    const NUM_RED  : usize = 12;
    const NUM_GREEN: usize = 13;
    const NUM_BLUE : usize = 14;
    let s = puzzle.games.iter()
        .filter(|g| {
            g.sets.iter().all(|s| (s.r <= NUM_RED) && (s.g <= NUM_GREEN) && (s.b <= NUM_BLUE))
        })
        .map(|g| g.id)
        .sum();
    Some(s)
}

fn part_2(puzzle: &Puzzle) -> Option<u32> {
    let get_power = |g: &Game| {
        let p = g.sets.iter().fold(Rgb::zero(), |s1, s2| Rgb::new(
            std::cmp::max(s1.r, s2.r),
            std::cmp::max(s1.g, s2.g),
            std::cmp::max(s1.b, s2.b)
        ));
        (p.r * p.g * p.b) as u32
    };
    let ps = puzzle.games.iter()
        .map(get_power)
        .sum();
    Some(ps)
}

fn solve_part_1(puzzle: &Puzzle) -> u32 {
    part_1(puzzle).expect("valid puzzle")
}

fn solve_part_2(puzzle: &Puzzle) -> u32 {
    part_2(puzzle).expect("valid puzzle")
}

pub fn solve() {
    let data = include_str!("../../data/day_02/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", solve_part_1(&puzzle));
    println!("part 2: {}", solve_part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    #[test]
    fn test_parse() {
        assert!(Puzzle::parse("").is_some());
        assert!(Puzzle::parse("Game 1: 3 blue, 4 red").is_some());
        assert!(Puzzle::parse("Game 1: 3 blue, 4 red; 1 red, 2 green").is_some());
        assert!(Puzzle::parse("Game 1: 3 blue, 4 black").is_none());
        assert!(Puzzle::parse("Game 1:").is_none());
        assert!(Puzzle::parse("Game 1: ;").is_none());
    }

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_1(&puzzle) == 8);
    }
        
    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert!(solve_part_2(&puzzle) == 2286);
    }
}
