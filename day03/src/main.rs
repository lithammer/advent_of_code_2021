fn part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let columns = lines[0].len();

    let mut gamma = 0;
    for column in 0..columns {
        gamma <<= 1;
        let ones = lines
            .iter()
            .map(|s| s.chars().nth(column).unwrap())
            .filter(|&c| c == '1')
            .count();
        if ones >= lines.len() - ones {
            gamma += 1;
        }
    }

    let epsilon = !gamma & ((1 << columns) - 1);
    gamma * epsilon
}

fn part2(input: &str, most: bool) -> u32 {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut column = 0;
    while lines.len() > 1 {
        let ones = lines
            .iter()
            .map(|s| s.chars().nth(column).unwrap())
            .filter(|&c| c == '1')
            .count();
        if ones >= (lines.len() - ones) {
            lines.retain(|x| x.chars().nth(column).unwrap() == if most { '1' } else { '0' })
        } else {
            lines.retain(|x| x.chars().nth(column).unwrap() == if most { '0' } else { '1' })
        }

        column += 1;
    }
    u32::from_str_radix(lines[0], 2).unwrap()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input, true) + part2(input, false));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{part1, part2};

    const INPUT: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT, true) * part2(INPUT, false), 230);
    }
}
