#[derive(Clone)]
struct Hand {
    cards: [u8; 5],
    bid: u32
}

impl Hand {
    fn score(&self) -> u8 {
        let mut rs = [0u8; 15];
        for c in self.cards {
            rs[c as usize] += 1;
        }
        rs.sort();
        match (rs[14], rs[13]) {
            (5, _) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            _      => 0
        }
    }

    fn score_jokers(&self) -> u8 {
        let mut rs = [0i8; 15];
        for c in self.cards {
            rs[c as usize] += if c == 0 {-1} else {1};
        }
        rs.sort();
        match (rs[14] + rs[0].abs(), rs[13]) {
            (5, _) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            _      => 0
        }
    }
}

struct Puzzle {
    hands: Vec<Hand>
}

fn get_card_value(c: char) -> Option<u8> {
    match c {
        '2'..='9' => Some(c as u8 - b'0'),
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _   => None
    }
}

impl Puzzle {
    fn parse(data: &str) -> Option<Self> {
        let parse_hand = |s: &str| {
            let (cs, b) = s.trim().split_once(char::is_whitespace)?;
            let mut cards = [0; 5];
            for (i, c) in cs.chars().enumerate() {
                cards[i] = get_card_value(c)?;
            }
            let bid = b.parse().ok()?;
            Some(Hand {cards, bid})
        };
        let hands: Option<_> = data.trim().lines().map(parse_hand).collect();
        let hands = hands?;
        Some(Self {hands})
    }

    fn load(data: &str) -> Self {
        Self::parse(data).expect("valid input")
    }
}

fn part_1(puzzle: &Puzzle) -> u32 {
    let mut hands = puzzle.hands.clone();
    hands.sort_by(|h1, h2| {
        let (r1, r2) = (h1.score(), h2.score());
        if r1 != r2 {
            r1.cmp(&r2)
        } else {
            h1.cards.cmp(&h2.cards)
        }
    });
    hands.into_iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bid).sum()
}

fn part_2(puzzle: &Puzzle) -> u32 {
    let mut hands = puzzle.hands.clone();
    for h in hands.iter_mut() {
        for c in h.cards.iter_mut() {
            if *c == 11 {
                *c = 0;
            }
        }
    }
    hands.sort_by(|h1, h2| {
        let (r1, r2) = (h1.score_jokers(), h2.score_jokers());
        if r1 != r2 {
            r1.cmp(&r2)
        } else {
            h1.cards.cmp(&h2.cards)
        }
    });
    hands.into_iter().enumerate().map(|(i, h)| (i + 1) as u32 * h.bid).sum()
}

pub(crate) fn solve() {
    let data = include_str!("../../data/day_07/input.txt");
    let puzzle = Puzzle::load(data);
    println!("part 1: {}", part_1(&puzzle));
    println!("part 2: {}", part_2(&puzzle));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    ";

    #[test]
    fn test_part_1() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(part_1(&puzzle), 6440);
    }

    #[test]
    fn test_part_2() {
        let puzzle = Puzzle::load(DATA);
        assert_eq!(part_2(&puzzle), 5905);
    }
}
