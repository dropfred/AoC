use std::collections::HashSet;

struct Card {
    numbers: Vec<u32>,
    winnings: HashSet<u32>
}
struct Puzzle {
    cards: Vec<Card>
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_card = |s: &str| {
            let (winnings, numbers) = s.split_once(": ")?.1.split_once(" | ")?;
            let numbers: Result<_, _> = numbers.split_whitespace().map(|s| s.trim().parse()).collect();
            let numbers = numbers.ok()?;
            let winnings: Result<_, _> = winnings.split_whitespace().map(|s| s.trim().parse()).collect();
            let winnings = winnings.ok()?;
            Some(Card {numbers, winnings})
        };
        let data = data.trim();
        let cards: Option<_> = data.trim().lines().map(|s| parse_card(s.trim())).collect();
        let cards = cards?;
        Some(Self {cards})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    puzzle.cards.iter().map(|c| {
        let ws = c.numbers.iter().filter(|n| c.winnings.contains(*n)).count();
        if ws > 0 {1 << (ws - 1)} else {0}
    }).sum()
}

fn part_2(puzzle: &Puzzle) -> u32 {
    let mut cs = vec![1; puzzle.cards.len()];
    for c in 0..cs.len() {
        let card = &puzzle.cards[c]; 
        let ws = card.numbers.iter().filter(|n| card.winnings.contains(*n)).count();
        for w in 0..ws {
            cs[c + w + 1] += cs[c];
        }
    }
    cs.iter().sum()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_04/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(part_1(&puzzle), 13);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(part_2(&puzzle), 30);
    }
}
