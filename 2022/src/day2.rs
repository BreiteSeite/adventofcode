use std::{char, fs::read_to_string};

pub fn day2() {
    let input = read_to_string("input/day2.txt").expect("Can not read input");

    let total_score: u32 = input
        .lines()
        .map(|line| {
            (
                EncryptedColumn(line.chars().nth(0).expect("can not access first column")),
                EncryptedColumn(line.chars().nth(2).expect("can not access second column")),
            )
        })
        .map(|(col1, col2)| {
            Game {
                opponent: col1.get_shape().unwrap(),
                own: col2.get_shape().unwrap(),
            }
            .total_score()
        })
        .fold(0, |acc, score| acc + score);

    println!("Total score: {total_score}");
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum HandShape {
    Rock,
    Scissors,
    Paper,
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Won,
    Lost,
    Draw,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Game {
    own: HandShape,
    opponent: HandShape,
}

impl Game {
    pub fn play(self: Self) -> GameResult {
        if self.own == self.opponent {
            return GameResult::Draw;
        }

        match (self.own, self.opponent) {
            (HandShape::Rock, HandShape::Scissors) => GameResult::Won,
            (HandShape::Scissors, HandShape::Paper) => GameResult::Won,
            (HandShape::Paper, HandShape::Rock) => GameResult::Won,
            _ => GameResult::Lost,
        }
    }

    pub fn score_for_handshape(shape: HandShape) -> u32 {
        return match shape {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        };
    }

    pub fn score_for_gameresult(result: GameResult) -> u32 {
        return match result {
            GameResult::Lost => 0,
            GameResult::Draw => 3,
            GameResult::Won => 6,
        };
    }

    pub fn total_score(self: &Self) -> u32 {
        return Game::score_for_gameresult(self.play()) + Game::score_for_handshape(self.own);
    }
}

struct EncryptedColumn(char);

impl EncryptedColumn {
    fn get_shape(self: Self) -> Option<HandShape> {
        match self.0 {
            'A' => Some(HandShape::Rock),
            'B' => Some(HandShape::Paper),
            'C' => Some(HandShape::Scissors),
            'X' => Some(HandShape::Rock),
            'Y' => Some(HandShape::Paper),
            'Z' => Some(HandShape::Scissors),
            _ => None,
        }
    }
}

struct Round(String);

fn get_score(shape: HandShape) -> u8 {
    return match shape {
        HandShape::Rock => 1,
        HandShape::Paper => 2,
        HandShape::Scissors => 3,
    };
}

#[cfg(test)]
mod tests {
    use crate::day2::{get_score, EncryptedColumn, HandShape};

    use super::{Game, GameResult};

    #[test]
    fn test_get_score() {
        assert_eq!(1, get_score(HandShape::Rock));
        assert_eq!(2, get_score(HandShape::Paper));
        assert_eq!(3, get_score(HandShape::Scissors));
    }

    #[test]
    fn game() {
        assert_eq!(
            GameResult::Draw,
            Game {
                own: HandShape::Rock,
                opponent: HandShape::Rock
            }
            .play()
        );

        assert_eq!(
            GameResult::Lost,
            Game {
                own: HandShape::Paper,
                opponent: HandShape::Scissors
            }
            .play()
        );

        assert_eq!(
            GameResult::Lost,
            Game {
                own: HandShape::Scissors,
                opponent: HandShape::Rock
            }
            .play()
        );

        assert_eq!(
            GameResult::Won,
            Game {
                own: HandShape::Rock,
                opponent: HandShape::Scissors
            }
            .play()
        );
    }

    #[test]
    fn test_total_scores() {
        assert_eq!(
            8,
            Game {
                own: HandShape::Paper,
                opponent: HandShape::Rock
            }
            .total_score()
        );

        assert_eq!(
            1,
            Game {
                own: HandShape::Rock,
                opponent: HandShape::Paper
            }
            .total_score()
        );

        assert_eq!(
            6,
            Game {
                own: HandShape::Scissors,
                opponent: HandShape::Scissors
            }
            .total_score()
        );
    }

    #[test]
    fn decrypt_column() {
        assert_eq!(HandShape::Rock, EncryptedColumn('A').get_shape().unwrap());
        assert_eq!(HandShape::Rock, EncryptedColumn('X').get_shape().unwrap());
        assert_eq!(HandShape::Paper, EncryptedColumn('B').get_shape().unwrap());
        assert_eq!(HandShape::Paper, EncryptedColumn('Y').get_shape().unwrap());
        assert_eq!(
            HandShape::Scissors,
            EncryptedColumn('C').get_shape().unwrap()
        );
        assert_eq!(
            HandShape::Scissors,
            EncryptedColumn('Z').get_shape().unwrap()
        );
    }
}
