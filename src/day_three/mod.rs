use std::collections::HashSet;

use color_eyre::{Report, Result};

fn part_two_from_iter<I>(mut input_lines: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut badges = Vec::new();
    while let (Some(first), Some(second), Some(third)) =
        (input_lines.next(), input_lines.next(), input_lines.next())
    {
        let first_set =
            first
                .trim_end()
                .trim_start()
                .chars()
                .fold(HashSet::new(), |mut acc, next| {
                    acc.insert(next);
                    acc
                });
        let second_set =
            second
                .trim_end()
                .trim_start()
                .chars()
                .fold(HashSet::new(), |mut acc, next| {
                    acc.insert(next);
                    acc
                });
        let third_set =
            third
                .trim_end()
                .trim_start()
                .chars()
                .fold(HashSet::new(), |mut acc, next| {
                    acc.insert(next);
                    acc
                });

        let mut badge_iter = third_set
            .into_iter()
            .filter(|third_char| first_set.contains(third_char) && second_set.contains(third_char));

        match badge_iter.next() {
            Some(next_badge) => badges.push(next_badge),
            None => return Err(Report::msg("Failed to find badge in group")),
        }
        if badge_iter.next().is_some() {
            return Err(Report::msg("More than one badge type found"));
        }
    }

    let mut running_sum = 0;
    for badge in badges.into_iter() {
        running_sum += priority_of(badge)?;
    }
    Ok(running_sum)
}
fn part_one_from_iter<I>(input_lines: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut sum_priority = 0;
    for line in input_lines {
        let line_trimmed = line.trim_start().trim_end();

        if line_trimmed.len() % 2 != 0 {
            return Err(Report::msg("It said the have the same number of items in each part so I think this has be even.  At leats if I am saying that the priority doesn't matter with this calculation"));
        }
        let left = &line_trimmed[..line_trimmed.len() / 2];
        let right = &line_trimmed[line_trimmed.len() / 2..];

        // I'm hoping I can get away with just using char here instead of having to do some string shenanigans
        // Since this is just ascii
        let right = right.chars().fold(HashSet::new(), |mut acc, next| {
            acc.insert(next);
            acc
        });

        let in_both = left
            .chars()
            .filter(|left_char| right.contains(left_char))
            .fold(HashSet::new(), |mut acc, next| {
                acc.insert(next);
                acc
            });

        for item in in_both {
            sum_priority += priority_of(item)?;
        }
    }
    Ok(sum_priority)
}

fn priority_of(item: char) -> Result<usize> {
    if !item.is_ascii() {
        Err(Report::msg("Items are expected to be ascii"))
    } else if item.is_ascii_lowercase() {
        Ok(item as usize - 96)
    } else {
        Ok(item as usize - 38)
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one_from_iter, part_two_from_iter, priority_of};

    #[test]
    fn test_priority_of() {
        assert_eq!(priority_of('a').unwrap(), 1);
        assert_eq!(priority_of('z').unwrap(), 26);
        assert_eq!(priority_of('A').unwrap(), 27);
        assert_eq!(priority_of('Z').unwrap(), 52);
    }

    #[test]
    fn test_part_one_example() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = part_one_from_iter(input.split('\n').map(|x| x.to_string()));

        match result {
            Ok(result) => assert_eq!(result, 157),
            Err(err) => panic!("failed with err {:?}", err),
        }
    }

    #[test]
    fn test_part_one_my_input() {
        let input = crate::read_file_line_by_line("src/day_three/input.txt").unwrap();
        let result = part_one_from_iter(input);

        match result {
            Ok(result) => assert_eq!(result, 8515),
            Err(err) => panic!("failed with err {:?}", err),
        }
    }

    #[test]
    fn test_part_two_example() {
        let group_one = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg"
            .split('\n')
            .map(|x| x.to_string());

        let group_two = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
            .split('\n')
            .map(|x| x.to_string());

        let result = part_two_from_iter(group_one);
        match result {
            Ok(result) => assert_eq!(result, 18),
            Err(err) => panic!("failed with err {:?}", err),
        }

        let result = part_two_from_iter(group_two);
        match result {
            Ok(result) => assert_eq!(result, 52),
            Err(err) => panic!("failed with err {:?}", err),
        }

        let both_groups = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
            .split('\n')
            .map(|x| x.to_string());

        let result = part_two_from_iter(both_groups);
        match result {
            Ok(result) => assert_eq!(result, 70),
            Err(err) => panic!("failed with err {:?}", err),
        }
    }

    #[test]
    fn test_part_two_my_input() {
        let input = crate::read_file_line_by_line("src/day_three/input.txt").unwrap();
        let result = part_two_from_iter(input);

        match result {
            Ok(result) => assert_eq!(result, 2434),
            Err(err) => panic!("failed with err {:?}", err),
        }
    }
}
