use std::collections::HashSet;

advent_of_code::solution!(3);

trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        !(self.is_ascii_digit() || *self == '.')
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let sch: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = sch.len();
    let cols = sch[0].len();
    let mut sum = 0;

    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for row in 0..rows {
        for col in 0..cols {
            if sch[row][col].is_symbol() {
                let mut part_numbers = HashSet::new();
                for (n_row, n_col) in neighbors.map(|(d_row, d_col)| {
                    ((row as i32 + d_row) as usize, (col as i32 + d_col) as usize)
                }) {
                    if sch[n_row][n_col].is_ascii_digit() {
                        let mut start_col = n_col;
                        while start_col > 0 && sch[n_row][start_col - 1].is_ascii_digit() {
                            start_col -= 1;
                        }
                        let mut end_col = n_col;
                        while end_col < cols && sch[n_row][end_col].is_ascii_digit() {
                            end_col += 1;
                        }
                        let mut part_number = String::new();
                        for i_col in start_col..end_col {
                            part_number.push(sch[n_row][i_col]);
                        }
                        let part_number: u32 = part_number.parse().unwrap();
                        part_numbers.insert(part_number);
                    }
                }
                sum += part_numbers.iter().sum::<u32>();
            }
        }
    }

    sum.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let sch: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = sch.len();
    let cols = sch[0].len();
    let mut sum = 0;

    let neighbors = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for row in 0..rows {
        for col in 0..cols {
            if sch[row][col] == '*' {
                let mut part_numbers = HashSet::new();
                for (n_row, n_col) in neighbors.map(|(d_row, d_col)| {
                    ((row as i32 + d_row) as usize, (col as i32 + d_col) as usize)
                }) {
                    if sch[n_row][n_col].is_ascii_digit() {
                        let mut start_col = n_col;
                        while start_col > 0 && sch[n_row][start_col - 1].is_ascii_digit() {
                            start_col -= 1;
                        }
                        let mut end_col = n_col;
                        while end_col < cols && sch[n_row][end_col].is_ascii_digit() {
                            end_col += 1;
                        }
                        let mut part_number = String::new();
                        for i_col in start_col..end_col {
                            part_number.push(sch[n_row][i_col]);
                        }
                        let part_number: u32 = part_number.parse().unwrap();
                        part_numbers.insert(part_number);
                    }
                }
                if part_numbers.len() == 2 {
                    sum += part_numbers.iter().product::<u32>();
                }
            }
        }
    }

    sum.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "467835");
    }
}
