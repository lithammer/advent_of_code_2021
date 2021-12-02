fn part1(input: &str) -> i64 {
    let (x, y) = input
        .lines()
        .map(|s| s.split_once(' ').expect("unexpected input"))
        .map(|(a, b)| (a, b.parse::<i64>().expect("not a number")))
        .fold((0, 0), |(x, y), (direction, n)| match direction {
            "forward" => (x + n, y),
            "up" => (x, y - n),
            "down" => (x, y + n),
            _ => panic!("unknown direction"),
        });
    x * y
}

fn part2(input: &str) -> i64 {
    let (x, y, _) = input
        .lines()
        .map(|s| s.split_once(' ').expect("unexpected input"))
        .map(|(a, b)| (a, b.parse::<i64>().expect("not a number")))
        .fold((0, 0, 0), |(x, y, a), (direction, n)| match direction {
            "forward" => (x + n, y + a * n, a),
            "up" => (x, y, a - n),
            "down" => (x, y, a + n),
            _ => panic!("unknown direction"),
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

    use crate::{part1, part2};

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
