#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    game_numbers: Vec<u32>,
}

impl Card {
    fn winner(&self) -> usize {
        let mut count = 0;
        for guess in &self.winning_numbers {
            if self.game_numbers.contains(&guess) {
                count += 1;
            }
        }

        let base: usize = 2;
        if count == 0 {
            return 0;
        }
        base.pow(count - 1)
    }
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.lines() {
        let mut card = Card::default();

        let (id_part, numbers_part) = line.split_once(":").unwrap();
        card.id = id_part
            .strip_prefix("Card")
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        let (winning_numbers_part, gamen_numbers_part) = numbers_part.split_once("|").unwrap();

        card.winning_numbers = winning_numbers_part
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        card.game_numbers = gamen_numbers_part
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        cards.push(card);
    }
    cards
}

fn part_1(input: &str) -> usize {
    let cards = parse_input(input);

    let total: usize = cards.iter().map(|c| c.winner()).sum();
    total
}

fn part_2(_input: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_winner() {
        let card = Card {
            id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            game_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(card.winner(), 8)
    }
    #[test]
    fn test_part_1() {
        let input = include_str!("../test-input.txt");

        assert_eq!(part_1(input), 13);
    }
}
