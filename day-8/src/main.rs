#![feature(test)]

use std::cell::{RefCell};
use std::collections::{HashMap};
use std::hash::{Hash};
use std::iter;
use std::rc::{ Rc, Weak };
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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    Above,
    Right,
    Below,
    Left
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Tree {
    height: u8,
}

struct ForestGridTree {
    height: u8,
    connections: HashMap<Direction, Weak<RefCell<ForestGridTree>>>,
}

impl ForestGridTree {
    fn new(height: u8) -> Self {
        return ForestGridTree { height, connections: HashMap::new() }
    }

    fn add_connection(&mut self, direction: Direction, tree: Weak<RefCell<ForestGridTree>>) {
        self.connections.insert(direction, tree);
    }
}

pub trait TreeVisibility {
    type VisibleTrees: Iterator<Item = Option<Tree>>;
    type ViewingDistances: Iterator<Item = (Direction, usize)>;

    fn get_trees_in_direction(&self, direction: &Direction) -> Self::VisibleTrees;
    fn is_visible_from(&self, direction: &Direction) -> bool;
    fn get_viewing_distance(&self, direction: &Direction) -> usize;
    fn get_viewing_distances(&self) -> Self::ViewingDistances;
}

impl TreeVisibility for ForestGridTree {
    type VisibleTrees = Box<dyn Iterator<Item = Option<Tree>>>;
    type ViewingDistances = Box<dyn Iterator<Item = (Direction, usize)>>;

    fn get_trees_in_direction(&self, direction: &Direction) -> Self::VisibleTrees {
        match self.connections.get(direction) {
            Some(tree) => {
                match tree.upgrade() {
                    Some(t) => {
                        let current_tree = t.borrow();
                        return Box::new(iter::once(Some(Tree { height: current_tree.height })).chain(current_tree.get_trees_in_direction(direction)));
                    }
                    None => Box::new(iter::once(None))
                }
            },
            None => {
                return Box::new(iter::once(None));
            }
        }
    }

    fn get_viewing_distance(&self, direction: &Direction) -> usize {
        let trees_in_direction: Vec<Tree> = self.get_trees_in_direction(direction).filter_map(|p| p).collect::<Vec<Tree>>();
        let number_of_visible_trees = trees_in_direction.iter().take_while(|p| p.height < self.height).count();

        if number_of_visible_trees < trees_in_direction.len() {
            return number_of_visible_trees + 1;
        }
        return number_of_visible_trees;
    }

    fn is_visible_from(&self, direction: &Direction) -> bool {
        return self.get_trees_in_direction(&direction).filter_map(|p|p).all(|t| t.height < self.height);
    }

    fn get_viewing_distances(&self) -> Self::ViewingDistances {
        let directions: [Direction; 4] = [Direction::Above, Direction::Right, Direction::Below, Direction::Left];
        return Box::new(directions.map(|d| (d, self.get_viewing_distance(&d))).into_iter());
    }
}

trait ScenicScore {
    fn get_scenic_score(&self) -> i64;
}

impl ScenicScore for ForestGridTree {
    fn get_scenic_score(&self) -> i64 {
        self.get_viewing_distances().map(|(_, distance)| distance as i64).product()
    }
}


struct ForestGrid {
    trees: Vec<Rc<RefCell<ForestGridTree>>>,
}

impl ForestGrid {
    fn from(input: &str) -> Self {
        let rows_of_trees = input.lines().map(|f|{
            return f.bytes().map(|c| Rc::new(RefCell::new(ForestGridTree::new(c - b'0')))).collect::<Vec<Rc<RefCell<ForestGridTree>>>>();
        }).collect::<Vec<Vec<Rc<RefCell<ForestGridTree>>>>>();

        let zipped = rows_of_trees.iter().zip(rows_of_trees.iter().skip(1)).map(|(tree_row, tree_row_below)| tree_row.iter().zip(tree_row_below));

        zipped.flat_map(|f|f).for_each(|(tree_above,tree_below)| {
            let tree_below_weak = Rc::downgrade(tree_below);
            let tree_above_weak = Rc::downgrade(tree_above);

            tree_above.borrow_mut().add_connection(Direction::Below, tree_below_weak);
            tree_below.borrow_mut().add_connection(Direction::Above, tree_above_weak);
        });

        rows_of_trees.iter().for_each(|tree_row| {
            tree_row.iter().zip(tree_row.iter().skip(1)).for_each(|(left_tree, right_tree)| {
                let tree_left_weak = Rc::downgrade(left_tree);
                let tree_right_weak = Rc::downgrade(right_tree);

                left_tree.borrow_mut().add_connection(Direction::Right, tree_right_weak);
                right_tree.borrow_mut().add_connection(Direction::Left, tree_left_weak);
            });
        });


        let flattened_trees =  rows_of_trees.into_iter().flat_map(|rows| rows).collect::<Vec<Rc<RefCell<ForestGridTree>>>>();
        return ForestGrid { trees: flattened_trees };
    }

    fn get_number_of_visible_trees(&self) -> usize {
        let directions: [Direction; 4] = [Direction::Above, Direction::Right, Direction::Below, Direction::Left];

        self.trees.iter().filter(|t| directions
            .iter()
            .any(|d| t
                .borrow()
                .is_visible_from(d))).count()
    }

    fn get_max_scenic_score(&self) -> i64 {
        return self.trees.iter().map(|t| t.borrow().get_scenic_score()).max().unwrap();
    }
}

struct Puzzle;

impl Puzzle {
    fn solve_part_1(input: &str) -> usize {
        let forest = ForestGrid::from(input);
        return forest.get_number_of_visible_trees();
    }

    fn solve_part_2(input: &str) -> i64 {
        let forest = ForestGrid::from(input);
        return forest.get_max_scenic_score();
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
        assert_eq!( Puzzle::solve_part_1(SAMPLE), 21);
    }

    #[test]
    fn test_part1_input() {
        assert_eq!(Puzzle::solve_part_1(INPUT), 1700);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!( Puzzle::solve_part_2(SAMPLE), 8);
    }

    #[test]
    fn test_part2_input(){
        assert_eq!( Puzzle::solve_part_2(INPUT), 470596);
    }

    #[test]
    fn can_calculate_viewing_distance_based_on_first_example_for_sample(){
        let forest = ForestGrid::from(SAMPLE);
        let tree = forest.trees[7].borrow();
        let total_viewing_distance: Vec<(Direction, usize)> = tree.get_viewing_distances().collect();

        assert_eq!(total_viewing_distance, vec![(Direction::Above, 1), (Direction::Right, 2), (Direction::Below, 2), (Direction::Left, 1)]);
    }

    #[test]
    fn can_calculate_scenic_score_based_on_first_example_for_sample(){
        let forest = ForestGrid::from(SAMPLE);
        let tree = forest.trees[7].borrow();
        assert_eq!(tree.get_scenic_score(), 4);
    }

    #[test]
    fn can_calculate_viewing_distance_based_on_second_example_for_sample(){
        let forest = ForestGrid::from(SAMPLE);
        let tree = forest.trees[17].borrow();
        let total_viewing_distance: Vec<(Direction, usize)> = tree.get_viewing_distances().collect();

        assert_eq!(total_viewing_distance, vec![(Direction::Above, 2), (Direction::Right, 2), (Direction::Below, 1), (Direction::Left, 2)]);
    }

    #[test]
    fn can_calculate_scenic_score_based_on_second_example_for_sample(){
        let forest = ForestGrid::from(SAMPLE);
        let tree = forest.trees[17].borrow();
        assert_eq!(tree.get_scenic_score(), 8);
    }

    #[test]
    fn parses_forest_from_sample(){
        let forest = ForestGrid::from(SAMPLE);
        assert_eq!(forest.trees.len(), 25);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| Puzzle::solve_part_1(INPUT));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| Puzzle::solve_part_2(INPUT));
    }
}


