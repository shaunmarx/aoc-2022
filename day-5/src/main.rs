extern crate regex;
use std::{path::PathBuf, collections::{VecDeque}};
use clap::Parser;
use regex::{Regex, Captures};
use std::fs::{ read_to_string };

fn main() {
    let args = Cli::parse();

    match read_to_string(&args.path) {
        Ok(content) => {
            match Puzzle::parse(content) {
                Ok(puzzle) => {

                    match puzzle.stacks.get_arranged_stacks(&puzzle.moves) {
                        Ok(mut arranged_stacks) => {
                            println!("{}", arranged_stacks.get_top_crates_str());
                        },
                        Err(err) => panic!("{}", err)
                    }
                },
                Err(err) => {
                    println!("{}", err);
                }
            }
        },
        Err(err) => {
            println!("Failed to read file {}", err)
        }
    }
}

struct Puzzle {
    stacks: Stacks,
    moves: Moves,
}

impl Puzzle {
    fn parse(content: String) -> Result<Puzzle, String>{
        let split_content: Vec<&str> = content.split("\n\n").collect();

        match split_content[..] {
            [stack_content, move_content] => {
                
                match (Stacks::parse(stack_content), Moves::parse(move_content)) {
                    (Ok(stacks), Ok(moves)) => {
                        return Ok(Puzzle { stacks:stacks, moves: moves })
                    }
                    _ => Err(String::from("Failed to parse puzzle")),
                }
            },
            _ => Err(String::from("Invalid puzzle content provided"))
        }
    }

}

struct Moves {
    items: Vec<Move>,
}

impl Moves {
    fn parse(value: &str) -> Result<Moves, String> {
        return Ok(Moves { items: value.split("\n").filter_map(|f| Move::parse(f).ok()).collect() });
    }
}

#[derive(Clone, Copy)]
struct Move {
    source: usize,
    destination: usize,
    number: usize,
}

impl Move {
    fn parse(value: &str) -> Result<Move, String> {

        let regex = Regex::new(r"move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)").unwrap();

        match regex.captures_iter(value).collect::<Vec<Captures>>().first() {
            Some(capture) => {
                match (&capture[1].parse::<usize>(), &capture[2].parse::<usize>(), &capture[3].parse::<usize>())
                {
                    (Ok(number), Ok(source), Ok(destination)) => {
                        return Ok(Move { number: number.clone(), source: source.clone(), destination: destination.clone() })
                    },
                    _ => Err(String::from(format!("Could not parse move {}", value )))
                }
            },
            None => return Err(String::from(format!("Could not parse move {}", value )))
        }
    }
}

struct Stacks {
    items: Vec<VecDeque<char>>
}

impl Stacks {
    fn parse(content: &str) -> Result<Stacks, String> {
        let mut crates: Vec<&str> = content.split("\n").collect::<Vec<&str>>();

        match crates.pop() { 
            Some(indexes) => {
                crates.reverse();
                let mut stacks: Vec<VecDeque<char>> = indexes.split_whitespace().map(|f| VecDeque::new()).collect();
        
                for &crate_line in crates.iter() {
                    let crate_chars: Vec<String> = crate_line
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|f| f.iter().filter(|f| !f.is_whitespace()) .collect::<String>())
                    .collect::<Vec<String>>();
        
                    crate_chars
                    .iter()
                    .map(|f|f.trim()).enumerate()
                    .for_each(|(index, crate_identifier)| 
                        if !crate_identifier.is_empty() {
                            stacks[index].push_back(crate_identifier.replace("[", "").replace("]", "").chars().last().unwrap()) 
                        }
                    );
                }
        
                return Ok(Stacks { items: stacks.into_iter().collect::<Vec<VecDeque<char>>>() });
            }
            None => Err(String::from("Provided input did not contain any elements"))
        }

    }

    pub fn get_arranged_stacks(&self, moves: &Moves) -> Result<Stacks, String> {
        
        let mut items: Vec<VecDeque<char>> = self.items.iter().map(|f|f.iter().cloned().collect::<VecDeque<char>>()).collect();
        
        for r#move in moves.items.iter(){
            let mut drained: VecDeque<char> = VecDeque::new();

            match items.get_mut(r#move.source - 1){
                Some(source) => {
                    for n in 0..r#move.number {
                        match source.pop_back() {
                            Some(item) => { drained.push_front(item) },
                            None => {
                                return Err(String::from("Number of moves exceeded stack"))
                            }
                        }
                    }
                },
                None => {
                    return Err(String::from(format!("Invalid source index provided for move: move {} from {} to {}", r#move.number, r#move.source, r#move.destination )));
                }
            }

            match items.get_mut(r#move.destination -1){
                Some(destination) => {
                    destination.append(&mut drained);
                },
                None => {
                    return Err(String::from(format!("Invalid destination index provided for move: move {} from {} to {}", r#move.number, r#move.source, r#move.destination )));
                }
            }
        }
          
        return Ok(Stacks { items });
    }

    pub fn get_top_crates(&mut self) -> Vec<char> {
        return self.items.iter().flat_map(|f| {
            return f.get(f.len()-1).cloned();
        }).collect();
    }

    pub fn get_top_crates_str(&mut self) -> String {
        return self.get_top_crates().iter().map(|c|c.to_string()).collect::<Vec<String>>().join("");
    }
}


#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

const SAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";


#[test]
fn correctly_arranges_stacks_based_on_moves(){
    let mut puzzle: Puzzle = Puzzle::parse(SAMPLE.to_string()).unwrap();
    match puzzle.stacks.get_arranged_stacks(&puzzle.moves) {
        Ok(mut arranged_stacks) => {
            assert_eq!(arranged_stacks.get_top_crates_str(), "MCD");
        },
        Err(err) => panic!("{}", err)
    }
}   

#[test]
fn parses_puzzle_with_correct_indexes(){
    let puzzle = Puzzle::parse(SAMPLE.to_string()).unwrap();
    assert_eq!(puzzle.stacks.items.iter().len(), 3)
}

#[test]
fn parses_puzzle_with_correct_stacks(){
    let puzzle = Puzzle::parse(SAMPLE.to_string()).unwrap();
    let items = &puzzle.stacks.items;
    match (items.get(0), items.get(1), items.get(2)) {
        (Some(first_stack), Some(second_stack), Some(third_stack)) => {
            assert_stack_values_eq(&first_stack, &[&'Z', &'N']);
            assert_stack_values_eq(&second_stack, &[&'M', &'C', &'D']);
            assert_stack_values_eq(&third_stack, &[&'P']);
        } 
        _ => { panic!("Invalid number of stacks.")}
    }
}

#[test]
fn parses_moves(){
    let puzzle = Puzzle::parse(SAMPLE.to_string()).unwrap();
    
    let expected = vec![
        Move{ number:1, source: 2, destination: 1}, 
        Move{ number: 3, source: 1, destination: 3 }, 
        Move{ number: 2, source: 2, destination: 1}, 
        Move{ number: 1, source: 1, destination: 2 }
    ];

    assert_moves_eq(expected, puzzle.moves.items);
}

fn assert_moves_eq(expected: Vec<Move>, received: Vec<Move>){
    assert_eq!(expected.len(), received.len(), "The number of moves do not match");

    expected.iter().zip(received.iter()).for_each(|(left, right)| assert_move_eq(left, right))
}


fn assert_move_eq(expected: &Move, received: &Move){
    assert_eq!(received.number, expected.number);
    assert_eq!(received.source, expected.source);
    assert_eq!(received.destination, expected.destination);
}

fn assert_stack_values_eq(stack: &VecDeque<char>, expected: &[&char]) {
    let converted: Vec<&char> = stack.iter().collect();
    assert_eq!(&converted[..], expected);
}

