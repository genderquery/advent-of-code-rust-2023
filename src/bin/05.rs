use std::{collections::HashMap, ops::Range};

advent_of_code::solution!(5);

type AggMap = Vec<(Range<u64>, Range<u64>)>;

fn parse_seeds(input: &str) -> Vec<u64> {
    input
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn parse_map(input: &str) -> AggMap {
    let mut lines = input.lines();
    // discard first line
    lines.next().unwrap();
    lines
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let dst: u64 = parts.next().unwrap().parse().unwrap();
            let src: u64 = parts.next().unwrap().parse().unwrap();
            let len: u64 = parts.next().unwrap().parse().unwrap();
            (src..src + len, dst..dst + len)
        })
        .collect()
}

fn parse(input: &str) -> (Vec<u64>, Vec<AggMap>) {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap());
    let mut maps: Vec<_> = parts.map(parse_map).collect();

    for map in maps.iter_mut() {
        map.sort_by_key(|(a, _)| a.start);
        let start = map.iter().next().unwrap().0.start;
        let end = map.iter().last().unwrap().0.end;
        let mut passthrough = HashMap::new();
        let mut windows = map.windows(2);
        while let Some([a, b]) = windows.next() {
            if a.0.end != b.0.start {
                passthrough.insert(a.0.end..b.0.start, a.0.end..b.0.start);
            }
        }
        map.extend(passthrough);
        if start > 0 {
            map.push((0..start, 0..start));
        }
        map.push((end..u64::MAX, end..u64::MAX));
        map.sort_by_key(|(a, _)| a.start);
    }

    (seeds, maps)
}

pub fn part_one(input: &str) -> Option<String> {
    let (seeds, maps) = parse(input);

    let mut lowest = u64::MAX;

    for seed in seeds.iter() {
        let mut value = *seed;
        for map in maps.iter() {
            for (src, dst) in map.iter() {
                if src.contains(&value) {
                    value = value - src.start + dst.start;
                    break;
                }
            }
        }
        if value < lowest {
            lowest = value;
        }
    }

    lowest.to_string().into()
}

pub fn part_two(input: &str) -> Option<String> {
    let (seeds, maps) = parse(input);
    let seeds: Vec<_> = seeds.chunks(2).map(|win| win[0]..win[0] + win[1]).collect();

    let mut lowest = u64::MAX;

    for seed in seeds.iter().cloned().flatten() {
        let mut value = seed;
        for map in maps.iter() {
            for (src, dst) in map.iter() {
                if src.contains(&value) {
                    value = value - src.start + dst.start;
                    break;
                }
            }
        }
        if value < lowest {
            lowest = value;
        }
    }

    lowest.to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        const INPUT: &str = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, "35");
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, "46");
    }
}
