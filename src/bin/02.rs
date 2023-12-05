use std::cmp;

use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    const RED_MAX: u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX: u32 = 14;

    let lines_re = Regex::new(r"(?m)^Game (\d+): (.*)$").unwrap();
    let pull_re = Regex::new(r"(\d+) ([[:word:]]+)").unwrap();

    let sum = lines_re.captures_iter(input)
        .map(|c| c.extract())
        .fold(0,|acc, (_, [id, pulls])| {
            let invalid = pull_re.captures_iter(pulls)
                .map(|c| c.extract())
                .any(|(_, [num, color])| {
                    let num = num.parse::<u32>().unwrap();
                    match color {
                        "red" => num > RED_MAX,
                        "green" => num > GREEN_MAX,
                        "blue" => num > BLUE_MAX,
                        _ => panic!("Invalid color found: {}", color)
                    }
                });
            if invalid {
                acc
            } else {
                acc  + id.parse::<u32>().unwrap()
            }
        }
    );
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines_re = Regex::new(r"(?m)^Game \d+: (.*)$").unwrap();
    let pull_re = Regex::new(r"(\d+) ([[:word:]]+)").unwrap();

    let sum = lines_re.captures_iter(input)
        .map(|c| c.extract())
        .fold(0,|acc, (_, [pulls])| {
            let (red, green, blue) = pull_re.captures_iter(pulls)
                .map(|c| c.extract())
                .fold((0, 0, 0), |(mut red_min, mut green_min, mut blue_min), (_, [num, color])| {
                    let num = num.parse::<u32>().unwrap();
                    match color {
                        "red" => red_min = cmp::max(num, red_min),
                        "green" => green_min = cmp::max(num, green_min),
                        "blue" => blue_min = cmp::max(num, blue_min),
                        _ => panic!("Invalid color found: {}", color)
                    }
                    (red_min, green_min, blue_min)
                });
           acc + red * green * blue
        });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
