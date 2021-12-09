fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split(',').map(|s| s.trim().parse().unwrap())
}

fn population(input: &str, days: usize) -> u64 {
    let fishes = parse_input(input);

    // Set starting population.
    let mut generations = [0; 9];
    for fish in fishes {
        generations[fish] += 1;
    }

    for _ in 1..=days {
        generations.rotate_left(1);
        generations[6] += generations[8];
    }

    generations.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", population(input, 80));
    println!("Part 2 = {}", population(input, 256));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const SAMPLE: &str = "3,4,3,1,2\n";

    #[test]
    fn test_part1() {
        assert_eq!(population(SAMPLE, 80), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(population(SAMPLE, 256), 26984457539);
    }
}
