use std::cmp::Ordering;
use std::path::PathBuf;
use clap::Parser;
use std::fs::{ read_to_string };
use std::collections::BinaryHeap;

fn main() {
    let args = Cli::parse();

    let content = read_to_string(&args.path);
    
    match content {
        Ok(value) => {
            let mut elves: Elves = parse_input(value);

            for elf in &elves.elves {
                println!("Elf {} has {} calories", elf.number, elf.calories );
            }

            let top_three_elves = elves.get_top_three_elves();
            let ranked_combined_calories: i32 = top_three_elves.iter().map(|e| e.calories).sum();

            for (index, ranked_elf) in top_three_elves.iter().enumerate() {
                println!("Ask elf # {} who ranked {} as he has {} calories worth of food", ranked_elf.number, index + 1, ranked_elf.calories);
            }

            println!("The combined total calories for the top 3 elves is {}", ranked_combined_calories);

        }
        Err(err) => {
            println!("Could not read file: {}", err)
        }
    }
}

fn parse_input(value: String) -> Elves {
    let lines: Vec<&str> = value.split("\n").collect();
    let segments = lines.split(|f| f.trim().is_empty());
    let elves = segments.into_iter().enumerate().map(|(index, items)| Elf { number: index + 1, calories: convert_calories(&items) });
    return Elves { elves: elves.collect() };
}

fn convert_calories(items: &[&str]) -> i32
{
    return items
        .into_iter()
        .filter_map(|item| item.parse::<i32>().ok())
        .sum()
}

struct Elves {
    elves: Vec<Elf>
}

#[derive(Clone, Copy)]
struct Elf {
    number: usize,
    calories: i32,
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.calories.cmp(&other.calories);
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        (self.number, &self.calories) == (other.number, &other.calories)
    }
}

impl Eq for Elf {}


impl Elves {
    fn get_top_three_elves(&mut self) -> Vec<Elf> {
        let mut heap = self.elves.iter().copied().collect::<BinaryHeap<Elf>>();
        let mut top_three = Vec::new();

        for _ in 0..3 {
            match heap.pop() {
                Some(elf) => {
                    top_three.push(elf);
                }
                None => {}
            }
        }

        return top_three;
    }
}

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}
