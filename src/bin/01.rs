use regex::{Regex, RegexSet};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Using loops
    // let mut sum = 0;
    // for line in input.split_terminator("\n") {
    //     let mut start = -1;
    //     let mut end = 0;
    //     for char in line.chars() {
    //         if char.is_ascii_digit() {
    //             let value = i32::try_from(char.to_digit(10).unwrap()).unwrap();
    //             if start == -1 {
    //                 start = value;
    //             }
    //             end = value;
    //         }
    //     }
    //     sum += start * 10 + end;
    // }
    // return Some(sum);

    // Using Regex
    let re = Regex::new(r"(?m)^\D*(\d)?.*(\d)\D*$").unwrap();
    let sum = re.captures_iter(input)
        .map(|c| (
            c.get(1).map(|m| str::parse::<u32>(m.as_str()).unwrap()),
            str::parse::<u32>(c.get(2).unwrap().as_str()).unwrap(),
        ))
        .fold(0, |acc, (start, end)| {
            acc + start.unwrap_or(end)*10 + end
        });
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mapper = |val| {
        match val {
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => str::parse::<u32>(val).unwrap()
        }
    };
    let re_first = Regex::new(r"^\D*?(\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_last = Regex::new(r"^[[:word:]]*(\d|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let sum = input.split_terminator("\n").into_iter()
        .map(|line|
            (
                mapper(re_first.captures(line).unwrap().get(1).unwrap().as_str()),
                mapper(re_last.captures(line).unwrap().get(1).unwrap().as_str())
            )
        ).fold(0, |acc, (start, end)| {
            acc + start*10 + end
        });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}