#![allow(unused)]

use std::collections::HashMap;

use winnow::{
    ascii::{dec_int, space0},
    combinator::{alt, separated},
    trace::trace,
    PResult, Parser,
};
use winnow::{prelude::*, seq};

fn parse_rounds(input: &mut &str) -> PResult<Vec<Round>> {
    trace("Rounds", separated(1.., parse_round, (space0, ';', space0))).parse_next(input)
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Default)]
struct Round {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Color {
    Red,
    Blue,
    Green,
}

fn parse_round(input: &mut &str) -> PResult<Round> {
    separated(
        1..=3,
        (
            dec_int,
            space0,
            trace(
                "Color",
                alt((
                    "red".value(Color::Red),
                    "green".value(Color::Green),
                    "blue".value(Color::Blue),
                )),
            ),
        )
            .map(|x| (x.2, x.0)),
        (space0, ',', space0),
    )
    .map(|x: HashMap<Color, i32>| Round {
        red: *x.get(&Color::Red).unwrap_or(&0),
        blue: *x.get(&Color::Blue).unwrap_or(&0),
        green: *x.get(&Color::Green).unwrap_or(&0),
    })
    .parse_next(input)
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let (game_id, rounds): (i32, _) = trace(
                "full thing",
                seq!(_: "Game ", dec_int, _: ": ", parse_rounds),
            )
            .parse(line)
            .unwrap();

            if !rounds
                .into_iter()
                .any(|Round { red, blue, green }| red > 12 || green > 13 || blue > 14)
            {
                return Some(game_id);
            }
            None
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (game_id, rounds): (i32, _) = trace(
                "full thing",
                seq!(_: "Game ", dec_int, _: ": ", parse_rounds),
            )
            .parse(line)
            .unwrap();

            let min_set = rounds
                .into_iter()
                .reduce(|acc, e| Round {
                    red: acc.red.max(e.red),
                    blue: acc.blue.max(e.blue),
                    green: acc.green.max(e.green),
                })
                .unwrap();

            min_set.red * min_set.green * min_set.blue
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
        assert_eq!(part1(test_input), 8);
    }

    #[test]
    fn test_part2() {
        let test_input = include_str!("testinput");
        assert_eq!(part2(test_input), 2286);
    }
}
