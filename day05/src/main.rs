use std::collections::HashMap;

#[derive(Default)]
struct Counter(HashMap<(u32, u32), u8>);

impl Counter {
    fn inc(&mut self, line: Line) -> u32 {
        let mut overlaps = 0;

        let Line { x1, x2, y1, y2 } = line;
        if x1 == x2 || y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    let v = self.0.entry((x, y)).or_insert(0);
                    *v += 1;
                    if v == &2 {
                        overlaps += 1;
                    }
                }
            }
        }

        overlaps
    }
}

#[derive(Debug)]
struct Line {
    x1: u32,
    x2: u32,
    y1: u32,
    y2: u32,
}

#[allow(dead_code)]
fn points(start: u32, stop: u32) -> impl Iterator<Item = u32> {
    (start..=stop).chain((stop..=start).rev())
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(a, b)| (a.split_once(',').unwrap(), b.split_once(',').unwrap()))
        .map(|((a, b), (c, d))| Line {
            x1: a.parse().unwrap(),
            x2: c.parse().unwrap(),
            y1: b.parse().unwrap(),
            y2: d.parse().unwrap(),
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let lines = parse_input(input);

    let mut counter = Counter::default();
    lines.into_iter().map(|l| counter.inc(l)).sum()
}

fn part2(_input: &str) -> u32 {
    todo!()
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
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 12);
    }
}
