#![feature(test)]
use clap::Parser;

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
    fn solve_part_1(input: &str) -> usize {
        const WINDOW_SIZE: usize = 4;

        return input.as_bytes().windows(WINDOW_SIZE).position(|x| {
            (x[0] != x[1] && x[0] != x[2] && x[0] != x[3])
            && (x[1] != x[2] && x[1] != x[3])
            && (x[2] != x[3])
        }).unwrap() + WINDOW_SIZE;
    }

    fn solve_part_2(input: &str) -> usize {
        const WINDOW_SIZE: usize = 14;

        return input.as_bytes().windows(WINDOW_SIZE).position(|x| {
            let mut seen = [false; 52];
            
            for &e in x {
                let index: usize = (if e >= 97 && e <= 122 { e - 97 } else { e - 65 + 26 }) as usize;
                
                if seen[index] {
                    return false;
                }

                seen[index] = true;
            }

            return true;

        }).unwrap() + WINDOW_SIZE;

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;


    #[test]
    fn test_part1_sample() {
        assert_eq!( Puzzle::solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!( Puzzle::solve_part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!( Puzzle::solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!( Puzzle::solve_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
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
        assert_eq!( Puzzle::solve_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!( Puzzle::solve_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!( Puzzle::solve_part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!( Puzzle::solve_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!( Puzzle::solve_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| Puzzle::solve_part_2(INPUT));
    }

}


