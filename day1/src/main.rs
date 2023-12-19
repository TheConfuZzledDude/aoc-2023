use winnow::{combinator::alt, stream::AsChar, token::take_while, trace::trace, PResult, Parser};

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter(char::is_ascii_digit);
            let first = digits.next().unwrap();
            let second = digits.last().unwrap_or(first);
            format!("{first}{second}").parse::<i32>().unwrap()
        })
        .sum()
}

fn parse_word(input: &mut &str) -> PResult<i32> {
    alt((
        "one".value(1),
        "two".value(2),
        "three".value(3),
        "four".value(4),
        "five".value(5),
        "six".value(6),
        "seven".value(7),
        "eight".value(8),
        "nine".value(9),
    ))
    .parse_next(input)
}

fn part2(input: &'static str) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut buffer: Vec<i32> = vec![];
            for index in 0..line.len() {
                let substring = &line[index..];
                if let Ok((_, n)) = trace(
                    "parser",
                    alt((
                        parse_word,
                        take_while(1, AsChar::is_dec_digit).try_map(str::parse),
                    )),
                )
                .parse_peek(substring)
                {
                    buffer.push(n);
                }
            }

            let first = buffer.first().unwrap();
            let second = buffer.last().unwrap_or(first);
            format!("{first}{second}").parse::<i32>().unwrap()
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
        assert_eq!(part1(test_input), 142);
    }

    #[test]
    fn test_part2() {
        let test_input2 = include_str!("testinput2");
        assert_eq!(part2(test_input2), 281);
    }
}
