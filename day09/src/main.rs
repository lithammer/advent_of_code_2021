use std::{collections::BTreeSet, convert::Infallible, str::FromStr};

type Point = (i32, i32); // x, y
type Height = u8;

struct Grid {
    points: Vec<Vec<Height>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn neighbours(&self, x: i32, y: i32) -> [Point; 4] {
        // [left, right, up, down]
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        offsets.map(|(xo, yo)| (x + xo, y + yo))
    }

    fn get(&self, x: i32, y: i32) -> Option<Height> {
        let in_bounds = x >= 0 && x < self.width && y >= 0 && y < self.height;
        in_bounds.then(|| self.points[y as usize][x as usize])
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .lines()
            .map(|s| s.chars().filter_map(to_digit_u8).collect())
            .collect::<Vec<Vec<Height>>>();
        let width = points[0].len() as i32;
        let height = points.len() as i32;
        Ok(Self {
            points,
            width,
            height,
        })
    }
}

fn to_digit_u8(c: char) -> Option<u8> {
    c.to_digit(10).map(|d| d as u8)
}

fn is_low_point(grid: &Grid, x: i32, y: i32) -> bool {
    grid.neighbours(x, y)
        .iter()
        .filter_map(|(x, y)| grid.get(*x, *y))
        .min()
        .unwrap()
        .cmp(&grid.get(x, y).unwrap())
        .is_gt()
}

fn low_points(grid: &Grid) -> impl Iterator<Item = Point> + '_ {
    grid.points
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, _)| {
            let (x, y) = (i as i32 % grid.width, i as i32 / grid.width);
            is_low_point(grid, x, y).then(|| (x, y))
        })
}

fn basin(grid: &Grid, x: i32, y: i32, visited: &mut BTreeSet<Point>) {
    if visited.insert((x, y)) {
        for (xp, yp) in grid.neighbours(x, y) {
            if let Some(h) = grid.get(xp, yp) {
                if h != 9 {
                    basin(grid, xp, yp, visited);
                }
            }
        }
    }
}

fn part1(input: &str) -> u64 {
    let grid = input.parse::<Grid>().unwrap();
    low_points(&grid)
        .map(|(x, y)| grid.get(x, y).unwrap() as u64)
        .map(|p| p + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    let mut basin_sizes = vec![];
    for (x, y) in low_points(&grid) {
        let mut visited = BTreeSet::new();
        basin(&grid, x, y, &mut visited);
        basin_sizes.push(visited.len());
    }
    basin_sizes.sort_unstable();
    basin_sizes
        .into_iter()
        .rev()
        .take(3)
        .reduce(|acc, n| acc * n)
        .unwrap_or(0)
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
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 1134);
    }
}
