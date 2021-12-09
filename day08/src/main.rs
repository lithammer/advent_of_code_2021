struct Entry<'a> {
    signals: Vec<&'a str>,
    output: Vec<&'a str>,
}

fn parse_line(s: &str) -> Entry {
    let (a, b) = s.split_once(" | ").unwrap();
    Entry {
        signals: a.split_whitespace().collect(),
        output: b.split_whitespace().collect(),
    }
}

fn parse_input(input: &str) -> Vec<Entry> {
    input.lines().map(parse_line).collect()
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .flat_map(|o| o.output)
        .map(|o| match o.len() {
            2 | 3 | 4 | 7 => 1,
            _ => 0,
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", part1(input));
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
}
