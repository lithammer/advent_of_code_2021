use std::{cmp::Ordering, collections::HashMap};

use sscanf::scanf;

type Vents = HashMap<(i32, i32), u8>;

trait Counter {
    fn inc(&mut self, line: Line);
}

impl Counter for Vents {
    fn inc(&mut self, Line { x1, x2, y1, y2 }: Line) {
        let dx = match x1.cmp(&x2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        let dy = match y1.cmp(&y2) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };

        let mut x = x1;
        let mut y = y1;

        *self.entry((x, y)).or_insert(0) += 1;
        while x != x2 || y != y2 {
            x += dx;
            y += dy;
            *self.entry((x, y)).or_insert(0) += 1;
        }
    }
}

struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn parse_input(input: &str) -> impl Iterator<Item = Line> + '_ {
    input
        .lines()
        .map(|s| scanf!(s, "{},{} -> {},{}", i32, i32, i32, i32).unwrap())
        .map(|(x1, y1, x2, y2)| Line { x1, x2, y1, y2 })
}

fn part1(input: &str) -> i32 {
    let lines = parse_input(input);

    let mut vents = Vents::default();
    lines
        .filter(|Line { x1, x2, y1, y2 }| x1 == x2 || y1 == y2) // Filter diagonal lines.
        .for_each(|line| vents.inc(line));

    vents.values().filter(|&v| *v >= 2).count() as i32
}

fn part2(input: &str) -> i32 {
    let lines = parse_input(input);

    let mut counter = Vents::default();
    lines.for_each(|l| counter.inc(l));

    counter.values().filter(|&v| *v >= 2).count() as i32
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
