use std::collections::HashSet;

use itertools::Itertools;
use winnow::{
    ascii::{dec_int, digit1, space0, space1},
    combinator::separated,
    seq, Parser,
};

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (winning, ticket): (HashSet<i64>, HashSet<i64>) = seq!(
                _: ("Card",space1, digit1::<_,()>, ":", space1),
                separated(1.., dec_int, space1 ),
                _:  (space0,  '|', space0 ),
                separated(1.., dec_int, space1 )
            )
            .parse(line)
            .unwrap();

            match winning.intersection(&ticket).count() {
                0 => 0,
                n => 1 << (n - 1),
            }
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut scratchcards = 0;
    let scratchcard_matches = input
        .lines()
        .map(|line| {
            let (winning, ticket): (HashSet<i64>, HashSet<i64>) = seq!(
                _: ("Card",space1, digit1::<_,()>, ":", space1),
                separated(1.., dec_int, space1 ),
                _:  (space0,  '|', space0 ),
                separated(1.., dec_int, space1 )
            )
            .parse(line)
            .unwrap();

            winning.intersection(&ticket).count()
        })
        .collect_vec();

    for card_number in 0..scratchcard_matches.len() {
        calculate_matches(card_number, &scratchcard_matches, &mut scratchcards)
    }

    scratchcards
}

fn calculate_matches(card_number: usize, scratchcard_matches: &[usize], scratchcards: &mut i32) {
    *scratchcards += 1;

    for number in card_number + 1..=card_number + scratchcard_matches[card_number] {
        calculate_matches(number, scratchcard_matches, scratchcards);
    }
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
        assert_eq!(part1(test_input), 13);
    }

    #[test]
    fn test_part2() {
        let test_input = include_str!("testinput");
        assert_eq!(part2(test_input), 30);
    }
}
