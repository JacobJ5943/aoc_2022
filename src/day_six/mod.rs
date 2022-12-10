#![allow(dead_code)]
use std::collections::{HashSet, VecDeque};

use color_eyre::{Report, Result};

fn part_one(input_line: String) -> Result<usize> {
    let mut chars_iter = input_line.chars();
    let mut first_three: VecDeque<char> = (&mut chars_iter).take(3).collect();
    if first_three.len() < 3 {
        return Err(Report::msg("Not enough chars in the communication stream"));
    }
    for (c_har, marker_index) in chars_iter.zip(4..) {
        first_three.push_back(c_har);
        // I didn't want to allocate a HashSet for each check
        if first_three[0] != first_three[1]
            && first_three[0] != first_three[2]
            && first_three[0] != first_three[3]
            && first_three[1] != first_three[2]
            && first_three[1] != first_three[3]
            && first_three[2] != first_three[3]
        {
            return Ok(marker_index);
        }
        first_three.pop_front().unwrap();
    }
    Err(Report::msg("no marker index found"))
}

fn part_two(input_line: String) -> Result<usize> {
    let mut chars_iter = input_line.chars();
    let mut checking_chars: VecDeque<char> = (&mut chars_iter).take(13).collect();
    if checking_chars.len() < 13 {
        return Err(Report::msg("Not enough chars in the communication stream"));
    }
    let mut hash_set = HashSet::new();
    for (c_har, marker_index) in chars_iter.zip(14..) {
        checking_chars.push_back(c_har);
        for c_har in checking_chars.iter() {
            hash_set.insert(*c_har);
        }
        if hash_set.len() == 14 {
            return Ok(marker_index);
        }
        checking_chars.pop_front().unwrap();
        hash_set.clear();
    }
    Err(Report::msg("no marker index found"))
}

#[cfg(test)]
mod tests {
    use crate::read_file_line_by_line;

    use super::*;

    #[test]
    fn test_part_one_example() {
        let _result = part_one("nnnn".to_string()).unwrap_err();

        assert_eq!(
            part_one("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()).unwrap(),
            5
        );
        assert_eq!(
            part_one("nppdvjthqldpwncqszvftbrmjlhg".to_string()).unwrap(),
            6
        );
        assert_eq!(
            part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()).unwrap(),
            10
        );
        assert_eq!(
            part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()).unwrap(),
            11
        );
    }

    #[test]
    fn test_part_one_my_input() {
        assert_eq!(
            part_one(
                read_file_line_by_line("src/day_six/input.txt")
                    .unwrap()
                    .next()
                    .unwrap()
            )
            .unwrap(),
            1920
        );
    }

    #[test]
    fn test_part_two_example() {
        assert_eq!(
            part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()).unwrap(),
            19
        );
        assert_eq!(
            part_two("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()).unwrap(),
            23
        );
        assert_eq!(
            part_two("nppdvjthqldpwncqszvftbrmjlhg".to_string()).unwrap(),
            23
        );
        assert_eq!(
            part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()).unwrap(),
            29
        );
        assert_eq!(
            part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()).unwrap(),
            26
        );
    }

    #[test]
    fn test_part_two_my_input() {
        assert_eq!(
            part_two(
                read_file_line_by_line("src/day_six/input.txt")
                    .unwrap()
                    .next()
                    .unwrap()
            )
            .unwrap(),
            2334
        );
    }
}
