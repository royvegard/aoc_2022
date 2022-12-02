use std::fs;

pub fn part_1(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut score = 0;

    for g in input {
        let game = Game::from_hands(&g);
        score += rock_paper_scissors(&game);
    }

    score
}

pub fn part_2(inp: String) -> i32 {
    let input = parse_input(inp);
    let mut score = 0;

    for g in input {
        let game = Game::from_strategy(&g);
        score += rock_paper_scissors(&game);
    }

    score
}

enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum Strategy {
    Loose,
    Draw,
    Win,
}

struct Game {
    opponent: Hand,
    me: Hand,
}

impl Game {
    fn from_hands(input: &str) -> Self {
        let hands: Vec<&str> = input.split(' ').collect();
        let mut game = Self {
            opponent: Hand::Rock,
            me: Hand::Rock,
        };
        game.opponent = match hands[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => Hand::Rock,
        };
        game.me = match hands[1] {
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            _ => Hand::Rock,
        };
        game
    }

    fn from_strategy(input: &str) -> Self {
        let hands: Vec<&str> = input.split(' ').collect();
        let mut game = Self {
            opponent: Hand::Rock,
            me: Hand::Rock,
        };
        game.opponent = match hands[0] {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => Hand::Rock,
        };

        let strategy = match hands[1] {
            "X" => Strategy::Loose,
            "Y" => Strategy::Draw,
            "Z" => Strategy::Win,
            _ => Strategy::Loose,
        };

        match game.opponent {
            Hand::Rock => {
                match strategy {
                    Strategy::Loose => game.me = Hand::Scissors,
                    Strategy::Draw => game.me = Hand::Rock,
                    Strategy::Win => game.me = Hand::Paper,
                };
            }
            Hand::Paper => {
                match strategy {
                    Strategy::Loose => game.me = Hand::Rock,
                    Strategy::Draw => game.me = Hand::Paper,
                    Strategy::Win => game.me = Hand::Scissors,
                };
            }
            Hand::Scissors => {
                match strategy {
                    Strategy::Loose => game.me = Hand::Paper,
                    Strategy::Draw => game.me = Hand::Scissors,
                    Strategy::Win => game.me = Hand::Rock,
                };
            }
        };

        game
    }
}

fn rock_paper_scissors(game: &Game) -> i32 {
    let mut score = match game.me {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };

    match game.opponent {
        Hand::Rock => {
            match game.me {
                Hand::Rock => score += 3,
                Hand::Paper => score += 6,
                Hand::Scissors => score += 0,
            };
        }
        Hand::Paper => {
            match game.me {
                Hand::Rock => score += 0,
                Hand::Paper => score += 3,
                Hand::Scissors => score += 6,
            };
        }
        Hand::Scissors => {
            match game.me {
                Hand::Rock => score += 6,
                Hand::Paper => score += 0,
                Hand::Scissors => score += 3,
            };
        }
    }

    score
}

fn parse_input(path: String) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Something went wrong")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = String::from("input/day_02_test");
        assert_eq!(part_1(input), 15);
    }

    #[test]
    fn part_two() {
        let input = String::from("input/day_02_test");
        assert_eq!(part_2(input), 12);
    }
}
