use std::path::PathBuf;
use clap::Parser;
use std::fs::{ read_to_string };

fn main() {
    let args = Cli::parse();
    let content = read_to_string(&args.path);

    match content {
        Ok(value) => {
            let games: Vec<Game> = parse_games(&value);
            let total_score = calculate_total_score(games);

            println!("The total score will be {}", total_score);

        }
        Err(err) => {
            println!("Could not read file: {}", err)
        }
    }
}

fn calculate_total_score(games: Vec<Game>) -> i32 {
    return games.iter().map(|game| game.get_total_score()).sum()
}

fn parse_games(value: &str) -> Vec<Game> {
    return value
        .split("\n")
        .filter_map(|game_line| {
            let moves: Vec<&str> = game_line.split_whitespace().collect();
            match moves[..] {
                [opponent_move_raw, outcome_raw] => {
                    let game = (Shape::parse(opponent_move_raw), Outcome::parse(outcome_raw));

                    match game {
                        (Some(opponent_move),  Some(expected_outcome)) => {
                            let counter_move = Shape::create_for_outcome(&expected_outcome, &opponent_move);
                            Some(Game{ opponent_move: opponent_move, counter_move: counter_move  })
                        },
                        _ => {
                            return None;
                        }
                    }
                },
                _ => None
            }
        }).collect();
}

struct Game {
    opponent_move: Shape,
    counter_move:  Shape,
}

impl Game {
    fn get_outcome(&self) -> Outcome{
        match self {
            Game { opponent_move: Shape::Rock, counter_move: Shape::Paper } => Outcome::Win,
            Game { opponent_move: Shape::Rock, counter_move: Shape::Rock } => Outcome::Draw,
            Game { opponent_move: Shape::Rock, counter_move: Shape::Scissors } => Outcome::Loss,
            Game { opponent_move: Shape::Paper, counter_move: Shape::Scissors } => Outcome::Win,
            Game { opponent_move: Shape::Paper, counter_move: Shape::Paper } => Outcome::Draw,
            Game { opponent_move: Shape::Paper, counter_move: Shape::Rock } => Outcome::Loss,
            Game { opponent_move: Shape::Scissors, counter_move: Shape::Rock } => Outcome::Win,
            Game { opponent_move: Shape::Scissors, counter_move: Shape::Scissors } => Outcome::Draw,
            Game { opponent_move: Shape::Scissors, counter_move: Shape::Paper } => Outcome::Loss,
        }
    }

    fn get_total_score(&self) -> i32 {
        let outcome = self.get_outcome_score();
        let shape_score = self.counter_move.get_score();
        return outcome + shape_score;
    }

    fn get_outcome_score(&self) -> i32 {
        match self.get_outcome() {
            Outcome::Win => return 6,
            Outcome::Draw => return 3,
            Outcome::Loss => return 0,
        }
    }

}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn parse(value: &str) -> Option<Outcome> {
        match value {
            "X" => return Some(Outcome::Loss),
            "Y" => return Some(Outcome::Draw),
            "Z" => return Some(Outcome::Win),
            _ => None
        }
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn parse(value: &str) -> Option<Shape> {
        match value.to_uppercase().as_str() {
            "A" => return Some(Shape::Rock),
            "B" => return Some(Shape::Paper),
            "C" => return Some(Shape::Scissors),
            _ => return None
        }
    }

    fn create_for_outcome(outcome: &Outcome, opponent_shape: &Shape) -> Shape {
        match outcome {
            Outcome::Win => {
                match opponent_shape {
                    Shape::Paper => Shape::Scissors,
                    Shape::Rock => Shape::Paper,
                    Shape::Scissors => Shape::Rock,
                }
            },
            Outcome::Draw => {
                match opponent_shape {
                    Shape::Paper => Shape::Paper,
                    Shape::Scissors => Shape::Scissors,
                    Shape:: Rock => Shape::Rock,
                }
            },
            Outcome::Loss => {
                match opponent_shape {
                    Shape::Paper => Shape::Rock,
                    Shape::Rock => Shape::Scissors,
                    Shape::Scissors => Shape::Paper
                }
            }
        }
    }

    fn get_score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}


#[derive(Parser)]
struct Cli {
    path: PathBuf,
}


#[test]
fn calculates_sample_score(){
    let sample = "A Y \n
    B X \n
    C Z";

    let games: Vec<Game> = parse_games(&sample);
    let total_score = calculate_total_score(games);

    assert_eq!(total_score, 12)
}