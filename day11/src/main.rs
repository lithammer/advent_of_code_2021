use std::{collections::HashSet, convert::Infallible, str::FromStr};

type Point = (i32, i32); // x, y
type Energy = u8;

struct Grid {
    points: Vec<Vec<Energy>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn neighbours(&self, x: i32, y: i32) -> [Point; 8] {
        #[rustfmt::skip]
        let offsets = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];
        offsets.map(|(xo, yo)| (x + xo, y + yo))
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Energy> {
        let in_bounds = x >= 0 && x < self.width && y >= 0 && y < self.height;
        in_bounds.then(|| &mut self.points[y as usize][x as usize])
    }

    fn size(&self) -> usize {
        (self.width * self.height) as usize
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .lines()
            .map(|s| {
                s.chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as Energy))
                    .collect()
            })
            .collect::<Vec<Vec<Energy>>>();
        let width = points[0].len() as i32;
        let height = points.len() as i32;
        Ok(Self {
            points,
            width,
            height,
        })
    }
}

fn increase_energy(grid: &mut Grid, x: i32, y: i32, flashes: &mut HashSet<Point>) {
    if let Some(e) = grid.get_mut(x, y) {
        match *e {
            0 if flashes.contains(&(x, y)) => (),
            9 => {
                *e = 0;
                flashes.insert((x, y));
                for (xs, ys) in grid.neighbours(x, y) {
                    increase_energy(grid, xs, ys, flashes);
                }
            }
            _ => *e += 1,
        }
    }
}

fn part1(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().unwrap();

    let mut total_flashes = 0;
    for _ in 0..100 {
        let mut flashes = HashSet::<Point>::new();

        for y in 0..grid.height {
            for x in 0..grid.width {
                increase_energy(&mut grid, x, y, &mut flashes);
            }
        }

        total_flashes += flashes.len();
    }
    total_flashes
}

fn part2(input: &str) -> u64 {
    let mut grid = input.parse::<Grid>().unwrap();

    let mut step = 1;
    loop {
        let mut flashes = HashSet::<Point>::new();

        for y in 0..grid.height {
            for x in 0..grid.width {
                increase_energy(&mut grid, x, y, &mut flashes);
            }
        }

        if flashes.len() == grid.size() {
            return step;
        }

        step += 1;
    }
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
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 195);
    }
}
