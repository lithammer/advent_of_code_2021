struct BingoBoard<'a> {
    slots: [[&'a str; 5]; 5],
}

impl BingoBoard<'_> {
    const MARK: &'static str = "X";

    fn try_mark(&mut self, number: &str) {
        self.slots
            .iter_mut()
            .flatten()
            .filter(|n| n == &&number)
            .for_each(|v| *v = BingoBoard::MARK);
    }

    fn has_bingo(&self) -> bool {
        let horizontal = (0..5).any(|y| (0..5).all(|x| self.slots[y][x] == BingoBoard::MARK));
        let vertical = (0..5).any(|x| (0..5).all(|y| self.slots[y][x] == BingoBoard::MARK));
        horizontal || vertical
    }

    fn score(&self) -> u32 {
        self.slots
            .iter()
            .flatten()
            .filter_map(|s| s.parse::<u32>().ok())
            .sum()
    }
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<BingoBoard>) {
    let mut parts = input.split("\n\n");
    let numbers: Vec<&str> = parts.next().unwrap().split(',').collect();

    let boards = parts
        .map(|s| {
            let b: [[&str; 5]; 5] = s
                .lines()
                .map(|s| s.split_whitespace().collect::<Vec<_>>().try_into().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            b
        })
        .map(|b| BingoBoard { slots: b })
        .collect::<Vec<BingoBoard>>();

    (numbers, boards)
}

fn part1(input: &str) -> Option<u32> {
    let (numbers, mut boards) = parse_input(input);
    for number in numbers {
        for board in boards.iter_mut() {
            board.try_mark(number);

            if board.has_bingo() {
                return Some(board.score() * number.parse::<u32>().unwrap());
            }
        }
    }

    None
}

fn part2(input: &str) -> Option<u32> {
    let (numbers, mut boards) = parse_input(input);

    for number in numbers {
        for board in boards.iter_mut() {
            board.try_mark(number);
        }

        if boards.len() == 1 && boards[0].has_bingo() {
            return Some(boards[0].score() * number.parse::<u32>().unwrap());
        }

        boards.retain(|b| !b.has_bingo());
    }

    None
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 = {}", part1(input).unwrap());
    println!("Part 2 = {}", part2(input).unwrap());
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::*;

    const SAMPLE: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "};

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE).unwrap(), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE).unwrap(), 1924);
    }
}
