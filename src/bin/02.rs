advent_of_code::solution!(2);

type Cube = (u32, String);
type CubeSet = Vec<Cube>;
type Bag = Vec<CubeSet>;
type Game = (u32, Bag);

fn parse_cube(s: &str) -> Cube {
    let (qty, color) = s.split_once(' ').unwrap();
    let qty: u32 = qty.parse().unwrap();
    (qty, color.to_owned())
}

fn parse_cube_set(s: &str) -> CubeSet {
    s.split(", ").map(parse_cube).collect()
}

fn parse_bag(s: &str) -> Bag {
    s.split("; ").map(parse_cube_set).collect()
}

fn parse_game(s: &str) -> Game {
    let (game_id, bag) = s.split_once(": ").unwrap();
    let game_id = game_id.strip_prefix("Game ").unwrap().parse().unwrap();
    let bag = parse_bag(bag);

    (game_id, bag)
}

fn parse(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

fn is_game_possible(game: &Game) -> Option<u32> {
    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;

    let (game_id, bag) = game;
    for set in bag {
        for (qty, color) in set {
            if (color == "red" && *qty > RED_MAX)
                || (color == "green" && *qty > GREEN_MAX)
                || (color == "blue" && *qty > BLUE_MAX)
            {
                return None;
            }
        }
    }

    Some(*game_id)
}

fn power_of_game(game: &Game) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    let (_, bag) = game;

    for set in bag {
        for (qty, color) in set {
            if color == "red" {
                min_red = std::cmp::max(min_red, *qty);
            } else if color == "green" {
                min_green = std::cmp::max(min_green, *qty);
            } else if color == "blue" {
                min_blue = std::cmp::max(min_blue, *qty);
            }
        }
    }

    min_red * min_green * min_blue
}

pub fn part_one(input: &str) -> Option<String> {
    let games = parse(input);
    let sum: u32 = games.iter().filter_map(is_game_possible).sum();
    sum.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let games = parse(input);
    let sum: u32 = games.iter().map(power_of_game).sum();
    sum.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "8");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "2286");
    }
}
