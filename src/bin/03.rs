use std::collections::HashSet;

advent_of_code::solution!(3);

type Schematic = Vec<Vec<char>>;

fn parse(input: &str) -> Schematic {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_symbol(c: char) -> bool {
    !(c.is_ascii_digit() || c == '.')
}

fn find_part_number(sch: &Schematic, row: usize, col: usize, cols: usize) -> u32 {
    let mut start_col = col;
    while start_col > 0 && sch[row][start_col - 1].is_ascii_digit() {
        start_col -= 1;
    }
    let mut part_number = String::new();
    let mut i_col = start_col;
    while i_col < cols && sch[row][i_col].is_ascii_digit() {
        part_number.push(sch[row][i_col]);
        i_col += 1;
    }
    part_number.parse().unwrap()
}

fn find_part_numbers(sch: &Schematic, row: usize, col: usize, cols: usize) -> Vec<u32> {
    #[rustfmt::skip]
    static NEIGHBORS: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];
    let mut part_numbers = HashSet::new();
    for (n_row, n_col) in NEIGHBORS
        .map(|(d_row, d_col)| ((row as i32 + d_row) as usize, (col as i32 + d_col) as usize))
    {
        if sch[n_row][n_col].is_ascii_digit() {
            let part_number = find_part_number(sch, n_row, n_col, cols);
            part_numbers.insert(part_number);
        }
    }
    part_numbers.into_iter().collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let sch = parse(input);
    let rows = sch.len();
    let cols = sch[0].len();
    let mut sum = 0;

    for row in 0..rows {
        for col in 0..cols {
            if is_symbol(sch[row][col]) {
                let part_numbers = find_part_numbers(&sch, row, col, cols);
                sum += part_numbers.iter().sum::<u32>();
            }
        }
    }

    sum.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let sch = parse(input);
    let rows = sch.len();
    let cols = sch[0].len();
    let mut sum = 0;

    for row in 0..rows {
        for col in 0..cols {
            if sch[row][col] == '*' {
                let part_numbers = find_part_numbers(&sch, row, col, cols);
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
