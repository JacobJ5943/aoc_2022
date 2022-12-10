#![allow(dead_code, unused)]

use color_eyre::Result;
use std::path::Path;

use crate::read_file_line_by_line;

fn part_one_from_path<P>(input_path: P) -> Result<usize>
where
    P: AsRef<Path>,
{
    let input_lines = read_file_line_by_line(input_path)?;
    part_one_from_iter(input_lines)
}
fn part_two_from_path<P>(input_path: P) -> Result<(usize, usize, usize)>
where
    P: AsRef<Path>,
{
    let input_lines = read_file_line_by_line(input_path)?;
    part_two_from_iter(input_lines)
}

fn part_two_from_iter<I>(input_lines: I) -> Result<(usize, usize, usize)>
where
    I: Iterator<Item = String>,
{
    let mut running_sum = None; // So that there can be elves with 0 calories
    let mut max_three: Vec<usize> = Vec::with_capacity(3);

    for line in input_lines.chain(vec!["\n".to_string()].into_iter()) {
        // This is so there will always be a last compare with None
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() {
            let parsed_value = trimmed_line.parse::<usize>()?;
            running_sum = running_sum.or(Some(0)).map(|sum| sum + parsed_value);
        } else {
            if max_three.len() < 3 {
                if let Some(running_sum) = running_sum {
                    insert_sorted(running_sum, &mut max_three);
                };
            } else if let Some(running_sum) = running_sum {
                update_max_three(running_sum, &mut max_three);
            }

            running_sum = None;
        }
    }

    if max_three.len() < 3 {
        todo!()
    } else {
        Ok((max_three[0], max_three[1], max_three[2]))
    }
}

fn insert_sorted(input_value: usize, input_vec: &mut Vec<usize>) {
    for index in 0..input_vec.len() {
        if &input_value < input_vec.get(index).unwrap() {
            input_vec.insert(index, input_value);
            return;
        }
    }
    input_vec.push(input_value);
}

fn update_max_three(number_to_compare: usize, max_three: &mut Vec<usize>) {
    let mut highest_index = None;
    for (index, value) in max_three.iter().enumerate() {
        match number_to_compare.cmp(value) {
            std::cmp::Ordering::Less => {
                break;
            }
            std::cmp::Ordering::Equal => {
                break;
            }
            std::cmp::Ordering::Greater => highest_index = Some(index),
        }
    }

    if let Some(index) = highest_index {
        max_three.remove(0);
        max_three.insert(index, number_to_compare);
    }
}

fn part_one_from_iter<I>(input_lines: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut running_sum = None;
    let mut max_so_far: Option<usize> = None;

    for line in input_lines.chain(vec!["\n".to_string()].into_iter()) {
        let trimmed_line = line.trim();

        if !trimmed_line.is_empty() {
            let line_parsed = trimmed_line.parse::<usize>()?;
            running_sum = running_sum
                .or(Some(0))
                .map(|running_sum| running_sum + line_parsed);
        } else {
            match (max_so_far, running_sum) {
                (Some(max_so_far_inner), Some(running_sum)) => {
                    max_so_far = Some(max_so_far_inner.max(running_sum));
                }
                (None, Some(running_sum)) => {
                    max_so_far = Some(running_sum);
                }
                (_, _) => {}
            }
            running_sum = None;
        }
    }

    match max_so_far {
        Some(max_so_far) => Ok(max_so_far),
        None => Err(color_eyre::Report::msg("no elves reported calories")),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        part_one_from_iter, part_one_from_path, part_two_from_iter, part_two_from_path,
        update_max_three,
    };

    #[test]
    fn test_part_one_example() {
        let input = r#"1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000"#
            .split('\n');

        let result = part_one_from_iter(input.into_iter().map(|x| x.to_string()));
        match result {
            Ok(result) => assert_eq!(result, 24000),
            Err(result) => panic!("Expected result Ok(24000) got {:?}", result),
        }
    }

    #[test]
    fn test_part_one_my_input() {
        let result = part_one_from_path("src/day_one/input.txt");
        match result {
            Ok(result) => assert_eq!(result, 75501),
            Err(result) => panic!("Expected result Ok(75501) got {:?}", result),
        }
    }

    #[test]
    fn test_update_max_three() {
        let mut max_three = vec![1, 2, 3];
        update_max_three(0, &mut max_three);
        assert_eq!(max_three, vec![1, 2, 3]);
        update_max_three(5, &mut max_three);
        assert_eq!(max_three, vec![2, 3, 5]);
        update_max_three(3, &mut max_three);
        assert_eq!(max_three, vec![3, 3, 5]);
        update_max_three(4, &mut max_three);
        assert_eq!(max_three, vec![3, 4, 5]);
    }

    #[test]
    fn test_part_two_example() {
        let input = r#"1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000"#
            .split('\n');

        let result = part_two_from_iter(input.into_iter().map(|x| x.to_string()));
        match result {
            Ok(result) => assert_eq!(result, (10000, 11000, 24000)),
            Err(result) => panic!("Expected result Ok((10000,11000,24000)) got {:?}", result),
        }
    }

    #[test]
    fn test_part_two_my_input() {
        let result = part_two_from_path("src/day_one/input.txt");
        match result {
            Ok(result) => {
                assert_eq!(result, (69997, 70096, 75501));
                assert_eq!((result.0 + result.1 + result.2), 215594);
            }
            Err(result) => panic!("Expected result Ok(75501) got {:?}", result),
        }
    }
}
