fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: &str) -> i32 {
    let mut positions = parse_input(input);

    positions.sort_unstable();
    let naive_median = positions[positions.len() / 2];
    positions.iter().map(|p| (p - naive_median).abs()).sum()
}

fn fuel_needed(distance: i32) -> i32 {
    let distance = distance.abs();
    (distance + 1) * distance / 2
}

fn total_fuel(positions: &[i32], target: i32) -> i32 {
    positions.iter().map(|p| fuel_needed(p - target)).sum()
}

fn part2(input: &str) -> i32 {
    let positions = parse_input(input);

    let avg = positions.iter().sum::<i32>() as f32 / positions.len() as f32;
    // The average gives us a float, test both the ceiling and floor to get the optimal fuel
    // consumption.
    let floor = avg.floor() as i32;
    let ceil = avg.ceil() as i32;
    total_fuel(&positions, floor).min(total_fuel(&positions, ceil))
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", part1(input));
    println!("Part 2 = {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14\n";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 168);
    }

    #[test]
    fn test_fuel_needed() {
        assert_eq!(fuel_needed(1), 1);
        assert_eq!(fuel_needed(2), 3);
        assert_eq!(fuel_needed(3), 6);
        assert_eq!(fuel_needed(4), 10);
        assert_eq!(fuel_needed(-4), 10);
    }
}
