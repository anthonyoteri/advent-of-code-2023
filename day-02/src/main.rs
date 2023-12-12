use std::cmp::max;

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    bags: Vec<Bag>,
}

impl Game {
    fn limit(&self, limit: &Bag) -> bool {
        for bag in &self.bags {
            if bag.red > limit.red || bag.green > limit.green || bag.blue > limit.blue {
                println!("Discard game {} because of bag {:?}", self.id, bag);
                return false;
            }
        }
        true
    }

    fn max(&self) -> Bag {
        let mut power = Bag::default();
        for bag in &self.bags {
            power.red = max(power.red, bag.red);
            power.green = max(power.green, bag.green);
            power.blue = max(power.blue, bag.blue);
        }
        power
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.lines() {
        let mut bags = Vec::new();

        let (game_part, bags_part) = line.split_once(':').unwrap();

        let game_id = game_part
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        for bag_input in bags_part.split(';') {
            let mut bag = Bag::default();

            for color_cube in bag_input
                .split(',')
                .map(|s| s.trim())
                .collect::<Vec<&str>>()
            {
                let (number, color) = color_cube.split_once(' ').unwrap();
                match color {
                    "red" => {
                        bag.red = number.parse::<u32>().unwrap();
                    }
                    "green" => {
                        bag.green = number.parse::<u32>().unwrap();
                    }
                    "blue" => {
                        bag.blue = number.parse::<u32>().unwrap();
                    }
                    _ => panic!("unknown color"),
                }
            }
            bags.push(bag);
        }
        games.push(Game { id: game_id, bags });
    }

    games
}

fn part_1(input: &str) -> usize {
    let games = parse_input(input);

    games
        .iter()
        .filter(|g| {
            g.limit(&Bag {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|g| g.id)
        .sum()
}

fn part_2(input: &str) -> u32 {
    let games = parse_input(input);

    games.iter().map(|g| g.max().power()).sum()
}
fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_parse() {
        let input = include_str!("../input_test_part1.txt");
        let games = parse_input(input);

        assert_eq!(games[0].id, 1);
        assert_eq!(
            games[0].bags,
            vec![
                Bag {
                    blue: 3,
                    red: 4,
                    ..Bag::default()
                },
                Bag {
                    green: 2,
                    blue: 6,
                    red: 1,
                },
                Bag {
                    green: 2,
                    ..Bag::default()
                }
            ]
        );

        assert_eq!(games[1].id, 2);
        assert_eq!(games[2].id, 3);
        assert_eq!(games[3].id, 4);
        assert_eq!(games[4].id, 5);
    }

    #[test]
    fn test_part_1_filter() {
        let input = include_str!("../input_test_part1.txt");
        let games = parse_input(input);

        let bag = Bag {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert!(games[0].limit(&bag));
        assert!(games[1].limit(&bag));
        assert!(!games[2].limit(&bag));
        assert!(!games[3].limit(&bag));
        assert!(games[4].limit(&bag));
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../input_test_part1.txt");

        assert_eq!(part_1(input), 8);
    }

    #[test]
    fn test_max() {
        let input = include_str!("../input_test_part1.txt");
        let games = parse_input(input);

        assert_eq!(
            games[0].max(),
            Bag {
                red: 4,
                green: 2,
                blue: 6
            }
        );

        assert_eq!(
            games[1].max(),
            Bag {
                red: 1,
                green: 3,
                blue: 4
            }
        );

        assert_eq!(
            games[2].max(),
            Bag {
                red: 20,
                green: 13,
                blue: 6
            }
        );

        assert_eq!(
            games[3].max(),
            Bag {
                red: 14,
                green: 3,
                blue: 15,
            }
        );

        assert_eq!(
            games[4].max(),
            Bag {
                red: 6,
                green: 3,
                blue: 2
            }
        );
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../input_test_part1.txt");

        assert_eq!(part_2(input), 2286);
    }
}
