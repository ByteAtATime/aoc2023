use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let nums = input.lines().map(|line| line.chars().filter(|c| c.is_digit(10)));

    let first_and_last_digit = nums.map(|num| (num.clone().next().unwrap(), num.clone().last().unwrap()));

    let sum = first_and_last_digit.map(|(first, last)| format!("{}{}", first, last).parse::<u32>().unwrap());

    Some(sum.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let word_to_digit = |word: &str| match word {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };

    let nums = input.lines().map(|line| {
        let mut digits = vec![];
        // either digit or word ("one")
        for (i, c) in line.char_indices() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap());
            }

            for j in 1..=9 {
                if i + j > line.len() {
                    break;
                }
                if let Some(digit) = word_to_digit(&line[i..i+j]) {
                    digits.push(digit);
                    break;
                }
            }
        }
        digits.into_iter()
    });

    let first_and_last_digit = nums.map(|num| (num.clone().next().unwrap(), num.clone().last().unwrap()));

    let sum = first_and_last_digit.map(|(first, last)| format!("{}{}", first, last).parse::<u32>().unwrap());

    Some(sum.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
