use std::error;
use std::fmt;

#[derive(Debug)]
enum MatchError {
    Corrupted(char),
}

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatchError::Corrupted(_) => write!(f, "corrupt pattern"),
        }
    }
}

impl error::Error for MatchError {}

fn is_matching_pair(a: char, b: char) -> bool {
    match a {
        '(' => b == ')',
        '[' => b == ']',
        '{' => b == '}',
        '<' => b == '>',
        _ => panic!("unexpected char: {}", a),
    }
}

fn match_parens(chars: impl Iterator<Item = char>) -> Result<Vec<char>, MatchError> {
    let mut stack: Vec<char> = vec![];
    for c in chars {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(v) if !is_matching_pair(v, c) => return Err(MatchError::Corrupted(c)),
                None => return Err(MatchError::Corrupted(c)),
                _ => (),
            },
            _ => panic!("unexpected char: {}", c),
        }
    }

    Ok(stack)
}

fn is_corrupted(s: &str) -> Option<char> {
    match_parens(s.chars()).err().map(|e| match e {
        MatchError::Corrupted(c) => c,
    })
}

fn error_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected char: {}", c),
    }
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(is_corrupted)
        .map(error_score)
        .sum()
}

fn autocomplete_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unexpected char: {}", c),
    }
}

fn part2(input: &str) -> u64 {
    let mut scores = input
        .lines()
        .filter_map(|s| match_parens(s.chars()).ok())
        .map(|cs| {
            cs.iter()
                .rfold(0, |acc, c| (acc * 5) + autocomplete_score(*c))
        })
        .collect::<Vec<u64>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::*;

    const SAMPLE: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 288957);
    }
}
