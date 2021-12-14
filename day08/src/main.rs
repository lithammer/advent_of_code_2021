use std::collections::BTreeSet;

type Signal = BTreeSet<char>;

fn parse_input(input: &str) -> impl Iterator<Item = (Vec<Signal>, Vec<Signal>)> + '_ {
    input.lines().map(|s| {
        let (a, b) = s.split_once(" | ").unwrap();

        let mut signals = a
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect::<Vec<Signal>>();
        signals.sort_by_key(|a| a.len());

        let outputs = b.split_whitespace().map(|s| s.chars().collect()).collect();

        (signals, outputs)
    })
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .flat_map(|(_, o)| o)
        .filter(|o| matches!(o.len(), 2 | 3 | 4 | 7))
        .count()
}

fn decode(signals: Vec<Signal>, outputs: Vec<Signal>) -> usize {
    let mut digits = (0..=9).map(|_| Signal::new()).collect::<Vec<Signal>>();
    digits[1] = signals[0].clone();
    digits[7] = signals[1].clone();
    digits[4] = signals[2].clone();
    digits[8] = signals[9].clone();

    #[allow(clippy::needless_range_loop)]
    for i in 6..=8 {
        if signals[i].difference(&digits[1]).count() == 5 {
            digits[6] = signals[i].clone();
        } else if signals[i].difference(&digits[4]).count() == 2 {
            digits[9] = signals[i].clone();
        } else {
            digits[0] = signals[i].clone();
        }
    }

    #[allow(clippy::needless_range_loop)]
    for i in 3..=5 {
        if signals[i].difference(&digits[1]).count() == 3 {
            digits[3] = signals[i].clone();
        } else if signals[i].difference(&digits[9]).count() == 0 {
            digits[5] = signals[i].clone();
        } else {
            digits[2] = signals[i].clone();
        }
    }

    let mut num = 0;
    for output in &outputs {
        for (i, digit) in digits.iter().enumerate() {
            if digit == output {
                num = num * 10 + i;
            }
        }
    }

    num
}

fn part2(input: &str) -> usize {
    parse_input(input).map(|(s, o)| decode(s, o)).sum()
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
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 61229);
    }
}
