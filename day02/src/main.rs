use std::{error, fmt, str::FromStr};

#[derive(Debug)]
enum ParseInstructionError {
    ParseIntError(std::num::ParseIntError, String),
    SplitError(String),
    UnknownAction(String),
}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParseIntError(s, e) => write!(f, "{}: {:?}", e, s),
            Self::SplitError(s) => write!(f, "failed to split string: {:?}", s),
            Self::UnknownAction(s) => write!(f, "unknown action in string: {:?}", s),
        }
    }
}

impl error::Error for ParseInstructionError {}

#[derive(Debug, PartialEq)]
enum Instruction {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, value) = s
            .split_once(' ')
            .ok_or_else(|| Self::Err::SplitError(s.to_owned()))?;
        let value = value
            .parse()
            .map_err(|e| Self::Err::ParseIntError(e, s.to_owned()))?;
        match action {
            "forward" => Ok(Self::Forward(value)),
            "up" => Ok(Self::Up(value)),
            "down" => Ok(Self::Down(value)),
            _ => Err(Self::Err::UnknownAction(action.to_owned())),
        }
    }
}

fn parse_instruction(s: &str) -> Instruction {
    s.parse::<Instruction>().unwrap()
}

fn part1(input: &str) -> u64 {
    let (x, y) = input
        .lines()
        .map(parse_instruction)
        .fold((0, 0), |(x, y), instruction| match instruction {
            Instruction::Forward(n) => (x + n, y),
            Instruction::Up(n) => (x, y - n),
            Instruction::Down(n) => (x, y + n),
        });
    x * y
}

fn part2(input: &str) -> u64 {
    let (x, y, _) =
        input
            .lines()
            .map(parse_instruction)
            .fold((0, 0, 0), |(x, y, a), instruction| match instruction {
                Instruction::Forward(n) => (x + n, y + a * n, a),
                Instruction::Up(n) => (x, y, a - n),
                Instruction::Down(n) => (x, y, a + n),
            });
    x * y
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{part1, part2, Instruction};

    #[test]
    fn test_instruction_fromstr() {
        assert_eq!(
            "forward 5".parse::<Instruction>().unwrap(),
            Instruction::Forward(5)
        );

        assert!("forward a".parse::<Instruction>().is_err());
        assert!("unknown 1".parse::<Instruction>().is_err());
    }

    #[test]
    fn test_part1() {
        let input = indoc! {"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "};
        assert_eq!(part1(input), 150);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "};
        assert_eq!(part2(input), 900);
    }
}
