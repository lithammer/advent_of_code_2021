fn part1(values: &[i64]) -> usize {
    values.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part2(values: &[i64]) -> usize {
    // let values = values
    //     .windows(3)
    //     .map(|w| w.iter().sum())
    //     .collect::<Vec<usize>>();
    // depth_increases(&values)

    // The middle values are shared, so we can just compare the first value on the left hand side
    // with the last value on the right hand side instead of summing.
    // 199  A
    // 200  A B
    // 208  A B C
    // 210    B C D
    // 200      C D
    // 207        D
    values.windows(4).filter(|w| w[0] < w[3]).count()
}

fn main() {
    let input = include_str!("input.txt");
    let values = input
        .lines()
        .map(|s| s.parse().expect("not a number"))
        .collect::<Vec<i64>>();

    let count = part1(&values);
    println!("Part 1 = {}", count);

    let count = part2(&values);
    println!("Part 2 = {}", count);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_count_depth_increases_part_1() {
        let count = part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(count, 7);
    }

    #[test]
    fn test_count_depth_increases_part_2() {
        let count = part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(count, 5);
    }
}
