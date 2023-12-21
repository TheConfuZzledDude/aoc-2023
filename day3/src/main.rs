#![allow(unused)]

use nalgebra::{vector, ClosedAdd, Vector2};
use std::{
    cmp::PartialEq,
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{Add, AddAssign},
};

fn get_adjacent_locations<T>(location: Vector2<T>) -> impl Iterator<Item = Vector2<T>>
where
    T: ClosedAdd + PartialEq + Debug + AddAssign + 'static,
    T: From<i8> + Copy,
{
    let adjacency_matrix: Vec<Vector2<i8>> = vec![
        vector!(1, 0),
        vector!(-1, 0),
        vector!(0, 1),
        vector!(0, -1),
        vector!(1, 1),
        vector!(1, -1),
        vector!(-1, 1),
        vector!(-1, -1),
    ];
    adjacency_matrix
        .into_iter()
        .map(move |x| location + vector!(T::from(x.x), T::from(x.y)))
}

fn part1(input: &str) -> i32 {
    let mut number_locations: HashMap<Vector2<i32>, usize> = HashMap::new();
    let mut found_numbers: Vec<(String, bool)> = Vec::new();
    let mut symbol_locations: HashSet<Vector2<i32>> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match char {
                '.' => {
                    continue;
                }
                '0'..='9' => {
                    if let Some(&index) = number_locations.get(&vector!(x - 1, y)) {
                        found_numbers[index].0.push(char);
                        number_locations.insert(vector!(x, y), index);
                    } else {
                        let index = found_numbers.len();
                        found_numbers.push((char.to_string(), false));
                        number_locations.insert(vector!(x, y), index);
                    }
                }
                symbol => {
                    symbol_locations.insert(vector!(x, y));
                }
            }
        }
    }

    for symbol_location in symbol_locations.into_iter() {
        for adjacent in get_adjacent_locations(symbol_location) {
            if let Some(number) = number_locations.get(&adjacent) {
                found_numbers[*number].1 = true;
            }
        }
    }

    found_numbers
        .into_iter()
        .filter_map(|(number, marked)| {
            if marked {
                Some(number.parse::<i32>().unwrap())
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut number_locations: HashMap<Vector2<i32>, usize> = HashMap::new();
    let mut found_numbers: Vec<String> = Vec::new();
    let mut axle_locations: HashSet<Vector2<i32>> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match char {
                '0'..='9' => {
                    if let Some(&index) = number_locations.get(&vector!(x - 1, y)) {
                        found_numbers[index].push(char);
                        number_locations.insert(vector!(x, y), index);
                    } else {
                        let index = found_numbers.len();
                        found_numbers.push(char.to_string());
                        number_locations.insert(vector!(x, y), index);
                    }
                }
                '*' => {
                    axle_locations.insert(vector!(x, y));
                }
                _ => {
                    continue;
                }
            }
        }
    }

    axle_locations
        .into_iter()
        .filter_map(|axle_location| {
            let mut adjacent_numbers = HashSet::new();
            for adjacent in get_adjacent_locations(axle_location) {
                if let Some(&number) = number_locations.get(&adjacent) {
                    adjacent_numbers.insert(number);
                }
            }
            if adjacent_numbers.len() == 2 {
                Some(
                    adjacent_numbers
                        .into_iter()
                        .map(|index| found_numbers[index].parse::<i32>().unwrap())
                        .product::<i32>(),
                )
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("input");

    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = include_str!("testinput");
        assert_eq!(part1(test_input), 4361);
    }

    #[test]
    fn test_part2() {
        let test_input = include_str!("testinput");
        assert_eq!(part2(test_input), 467835);
    }
}
