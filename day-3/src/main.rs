use std::{path::PathBuf, collections::{HashSet, HashMap}};
use clap::Parser;
use std::fs::{ read_to_string };

fn main() {
    let args = Cli::parse();
    let content = read_to_string(&args.path);

    match content {
        Ok(value) => {
            let rucksacks: Vec<RuckSack> = get_rucksacks(&value); 
            let total_score = group_and_score_rucksacks(rucksacks);

            println!("The sum of priority items is {}", total_score);

        }
        Err(err) => {
            println!("Could not read file: {}", err)
        }
    }
}


fn calculate_duplicate_priority_score(rucksacks: Vec<RuckSack>) -> i32 {
    return rucksacks.iter().map(|rucksack| rucksack.get_duplicate_score() ).map(|(overlap, score)| score).sum()
}

fn get_rucksacks(value: &str) -> Vec<RuckSack> {
    return value
        .split("\n")
        .filter_map(|line| {
            match line.trim() {
                trimmed_line if !trimmed_line.is_empty() => Some(RuckSack::create_with_two_compartments(trimmed_line)),
                _ => None
            }
        }).collect();
}

#[derive(Debug)]
struct RuckSack {
    compartments: Vec<Compartment>,
}

impl RuckSack {
    fn create_with_two_compartments(items: &str) -> RuckSack{
        let (first, second) = items.split_at(items.len() / 2);

        let result = RuckSack { compartments: vec![Compartment::create(first.clone()), Compartment::create(second.clone())] };
        return result;
    }

    pub fn get_duplicates(&self) -> Vec<char> {
        return get_flattened_duplicates(self.compartments.iter().map(|f| f.items.clone()).collect())
    }

    pub fn get_duplicate_score(&self) -> (Vec<char>, i32) {
        let duplicates = self.get_duplicates();
        let score = duplicates.iter().map(|item| get_char_priority(item)).sum();

        println!("Rugsack with items {} has duplicated {} with score {}", self.compartments.iter().map(|f|f.items.clone()).flatten().collect::<String>(), duplicates.iter().collect::<String>(), score);

        return (duplicates, score);
    }

    pub fn to_chars(&self) -> Vec<char> {
        return self.compartments.iter().flat_map(|c|c.items.clone()).collect();
    }

}

fn group_and_score_rucksacks(rucksacks: Vec<RuckSack>) -> i32 {
    let rucksack_items: Vec<Vec<char>> = rucksacks.iter().map(|r|r.to_chars()).collect();
    let item_groups: Vec<&[Vec<char>]> = rucksack_items.chunks(3).collect();

    let duplicates: Vec<char> = item_groups.iter().map(|group| get_duplicates_multiple(group.to_vec() )).flatten().collect();

    println!("{:?}", duplicates);

    return duplicates.iter().map(|c| get_char_priority(c)).sum();
}


pub fn get_flattened_duplicates(items: Vec<Vec<char>>) -> Vec<char> {
    let duplicates = get_duplicates(items);
    return duplicates.into_iter().flatten().collect();
}

pub fn get_duplicates_multiple(items: Vec<Vec<char>>) -> Vec<char> {
    let duplicates = get_duplicates(items);
    let mut flattened: Vec<char> =  get_duplicates(duplicates).into_iter().flatten().collect();
    flattened.sort();
    flattened.dedup();
    return flattened;
}

pub fn get_duplicates(items: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut all_items: Vec<HashSet<char>> = items.iter().map(|c| c.iter().cloned().collect() ).collect();
    
    let mut intersections: Vec<Vec<char>> = Vec::new();

    while let Some(compartment_items) = &all_items.pop(){
        for remaining_items_in_compartment in &all_items {
            let overlap = compartment_items.intersection(&remaining_items_in_compartment);
            intersections.push(overlap.cloned().collect());
        }
    }

    return intersections;
}


#[derive(Debug)]
struct Compartment {
    items: Vec<char>,
}

impl Compartment {
    fn create(items: &str) -> Compartment {
        let compartment =  Compartment { items: items.chars().collect()};
        return compartment;
    }
}

fn get_char_priority(value: &char) -> i32 {
    let ascii_value = *value as u8;
    match ascii_value  {
         _ if ascii_value >= 65 && ascii_value <= 90 => {
             return (ascii_value as i32) - 65 + 1 + 26
         },
         _ if ascii_value >= 97 && ascii_value <= 122 => {
             return (ascii_value as i32) - 97 + 1

         },
         _ => return 0
    }
}

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}


#[test]
fn calculates_sample_score(){
    let sample = "vJrwpWtwJgWrhcsFMMfFFhFp\n
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n
    PmmdzqPrVvPwwTWBwg\n
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n
    ttgJtRGJQctTZtZT\n
    CrZsJsPPZsGzwwsLwLmpwMDw\n";

    let rucksacks: Vec<RuckSack> = get_rucksacks(&sample);
    let total_score: i32 = calculate_duplicate_priority_score(rucksacks);
    assert_eq!(total_score, 157)
}

#[test]
fn calculates_group_badge(){
    let sample = "vJrwpWtwJgWrhcsFMMfFFhFp\n
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n
    PmmdzqPrVvPwwTWBwg\n
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n
    ttgJtRGJQctTZtZT\n
    CrZsJsPPZsGzwwsLwLmpwMDw\n";

    let rucksacks = get_rucksacks(&sample);
    let total_score: i32 = group_and_score_rucksacks(rucksacks);
    assert_eq!(total_score, 70)
}

#[test]
fn rucksack_creates_correct_compartments(){
    let rucksack = RuckSack::create_with_two_compartments("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    match &rucksack.compartments[..] {
        [first, second] => {
            let first_str: String = first.items.iter().collect();
            let second_str: String = second.items.iter().collect();

            assert_eq!(first_str, "jqHRNqRjqzjGDLGL");  //jNqGzHDRL
            assert_eq!(second_str, "rsFMfFZSrLrFZsSL"); //FrfMLsZ
    
        },
        _ => {
            panic!("This should never happen!")
        }
    }
}
