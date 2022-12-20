#![feature(test)]
use clap::Parser;
use std::collections::{ HashMap };

const INPUT: &str = include_str!("../input");

fn main() {
    let args = Cli::parse();

    match args {
        Cli { part: 1 } => println!("Answer for part 1 is {}", Puzzle::solve_part_1(INPUT)),
        Cli { part: 2 } => println!("Answer for part 2 is {}", Puzzle::solve_part_2(INPUT)),
        _ => panic!("Unknown part. Can either be 1 or 2")
    }
}


#[derive(Parser)]
struct Cli {
    part: usize,
}

struct Puzzle;

impl Puzzle {
    fn solve_part_1(input: &str) -> i32 {
        const MAX_SIZE_INCLUSIVE: i32 = 100000;
        return Puzzle::get_sizes(input).values().filter(|&&x| x <= MAX_SIZE_INCLUSIVE).sum();
    }

    fn solve_part_2(input: &str) -> i32 {
        const TOTAL_DISK_SPACE: i32 =  70000000;
        const REQUIRED_UNUSED_SPACE: i32 = 30000000;
        let required_space = TOTAL_DISK_SPACE - REQUIRED_UNUSED_SPACE;

        let sizes = Puzzle::get_sizes(input);
        let total_used_space = sizes.get(&vec![String::from("/")]).unwrap();
    
        let minimum_required_space = total_used_space - required_space;
        return *sizes.values().filter(|&&v| v >= minimum_required_space).min().unwrap();
    }

    fn get_sizes(input: &str) -> HashMap<Vec<String>, i32> {
        let mut cwd: Vec<String> = Vec::new();

        return input.lines().fold(HashMap::new(), | mut sizes, line| {
            match Line::parse(line) {
                Ok(Line::ChangeDirectory(path))  => {
                    if path == ".." {
                        cwd.pop().unwrap();
                    } else {
                        cwd.push(path.clone());
                        sizes.insert(cwd.clone(), 0);
                    }
                },
                Ok(Line::File(_, size)) => {
                    for i in (1..cwd.len() + 1).rev() {
                        *sizes.get_mut(&cwd[0..i]).unwrap() += size;
                    }
                },
                Ok(_) => {}, //Line::Directory
                Err(err) => panic!("{}", err)
            }
            return sizes;
        });
    }
}

enum Line {
    ListDirectory(),
    ChangeDirectory(String),
    Directory(String),
    File(String, i32),
}

impl Line {
    fn parse(input: &str) -> Result<Line, &'static str> {
        let segments: Vec<&str> = input.split_whitespace().map(|f|f.trim()).collect();

        match (segments.get(0), segments.get(1)){
            (Some(&first), Some(&second)) => {

                match first {
                    "$" =>  {
                        match second { 
                            "cd" => {
                                match segments.get(2) {
                                    Some(&path) => {
                                        return Ok(Line::ChangeDirectory(String::from(path)));
                                    },
                                    None => { return Err("Expected a path for change directory but there was none.") }
                                }
                            },
                            "ls" =>  Ok(Line::ListDirectory()),
                            _ => return Err("Unknown command provided")
                        }
                    }
                    "dir" => {
                        return Ok(Line::Directory(String::from(second)))
                    },
                    _ => {
                        return Ok(Line::File(String::from(second), first.parse::<i32>().unwrap()));
                    }
                }
            },
            _ => return Err("Unknown input line provided"),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    const SAMPLE: &str = include_str!("../sample");

    #[test]
    fn test_part1_sample() {
        assert_eq!( Puzzle::solve_part_1(SAMPLE), 95437);
    }

    #[test]
    fn test_part1_output() {
        let output = Puzzle::solve_part_1(INPUT);
        assert!(output > 0);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| Puzzle::solve_part_1(INPUT));
    }


    #[test]
    fn test_part2_sample() {
        assert_eq!( Puzzle::solve_part_2(SAMPLE), 24933642);
    }

    #[test]
    fn test_part2_output() {
        let output = Puzzle::solve_part_2(INPUT);
        assert!(output > 0);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| Puzzle::solve_part_2(INPUT));
    }
}


