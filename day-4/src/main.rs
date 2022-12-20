use std::{path::PathBuf, collections::{HashSet}, ops::Range };
use clap::Parser;
use std::fs::{ read_to_string };

fn main() {
    let args = Cli::parse();
    let content = read_to_string(&args.path);

    match content {
        Ok(value) => {

            let assignments = get_assignments(&value);
            let count = get_overlap_count(assignments);
            
            println!("The number of assignments where one range overlaps another is {}", count);

        }
        Err(err) => {
            println!("Could not read file: {}", err)
        }
    }
}


fn get_assignments(value: &str) -> Vec<Pair<ZoneRange>> {
    return value
        .split("\n")
        .filter_map(|line| Pair::parse_zone_ranges(line.trim()).ok())
        .collect();
}

fn get_fully_contained_assignment_count(assignments: Vec<Pair<ZoneRange>>) -> i32{
    return assignments
    .iter()
    .fold(0, |count, p| if p.zone_fully_contained_by_other() { count + 1 } else { count } );
}

fn get_overlap_count(assignments: Vec<Pair<ZoneRange>>) -> i32{
    return assignments
    .iter()
    .fold(0, | count, p | match p.zones_overlap() { 
        (true, _) => { return count + 1 },
        _ => return count,
    });
}


#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

#[derive(Clone, Copy)]
struct ZoneRange {
    start: i32,
    end: i32,
}

struct Pair<T>{
    x: T,
    y: T,
}

impl Pair<ZoneRange> {
    fn parse_zone_ranges(value: &str) -> Result<Pair<ZoneRange>, &str>{
        let zone_ranges_str: Vec<&str> = value.split(",").collect();
        let zone_ranges: Vec<ZoneRange> = zone_ranges_str.iter().filter_map(|r| ZoneRange::parse(r).ok()).collect();

        match &zone_ranges[..] {
            [first, second] => {
                return Ok(Pair { x: *first, y: *second });
            },
            _ => return Err("Invalid zone range provided")
        }
    }

    fn zone_fully_contained_by_other(&self) -> bool {
        let x: HashSet<i32> = self.x.to_range().collect();
        let y: HashSet<i32> = self.y.to_range().collect();

        return x.is_superset(&y) || y.is_superset(&x);
    }

    fn zones_overlap(&self) -> (bool, Vec<i32>)  {
        let x: HashSet<i32> = self.x.to_range().collect();
        let y: HashSet<i32> = self.y.to_range().collect();

        let intersect: Vec<i32> = x.intersection(&y).copied().collect();
        return (intersect.len() > 0, intersect);
    }
}


impl ZoneRange {
    fn parse(zones: &str) -> Result<ZoneRange, String> {
        let segments: Vec<&str> = zones.split("-").collect();
        match segments[..] {
            [start, end] => {
                let converted = (start.parse::<i32>(), end.parse::<i32>());
                
                match converted {
                    (Ok(start_converted), Ok(end_converted)) => {
                        return Ok(ZoneRange { start: start_converted, end: end_converted })
                    },
                    _ => { Err(format!("Failed to parse range {}", zones))}
                }

                
            },
            _ => { return Err(format!("Unable to parse range {} as it didn't appear to be an actual range", zones)) }
        }
    }
    
    fn to_range(&self) -> Range<i32> {
        return self.start..self.end + 1;
    }

}


#[test]
fn calculates_fully_contained_count(){
    let sample = "2-4,6-8\n
    2-3,4-5\n
    5-7,7-9\n
    2-8,3-7\n
    6-6,4-6\n
    2-6,4-8";

    let assignments = get_assignments(&sample);
    let count = get_fully_contained_assignment_count(assignments);

    assert_eq!(count, 2)
}

#[test]
fn calculates_overlap_count(){
    let sample = "2-4,6-8\n
    2-3,4-5\n
    5-7,7-9\n
    2-8,3-7\n
    6-6,4-6\n
    2-6,4-8";

    let assignments = get_assignments(&sample);
    let count = get_overlap_count(assignments);

    assert_eq!(count, 4)
}