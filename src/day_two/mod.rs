use std::path::Path;

use crate::read_file_line_by_line;


use color_eyre::Result;

fn part_one_file<P>(input_path:P) -> Result<usize> where P:AsRef<Path> {
    part_one(read_file_line_by_line(input_path)?)
}
fn part_two_file<P>(input_path:P) -> Result<usize> where P:AsRef<Path> {
    part_two(read_file_line_by_line(input_path)?)
}

fn part_one<I>(input: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut running_sum = 0;
    for round in input {
        if !round.is_empty() {
            let split = round.split(' ').take(2).collect::<Vec<&str>>();
            let their_move = split[0];
            let my_move = split[1];
            running_sum += run_match(their_move, my_move);
        }
    }
    Ok(running_sum)
}

fn run_match(their_move:&str, my_move:&str) -> usize{
    match (their_move, my_move) {
        ("A", "X") => 4,
        ("A", "Y") => 8,
        ("A", "Z") => 3,
        ("B", "X") => 1,
        ("B", "Y") => 5,
        ("B", "Z") => 9,
        ("C", "X") => 7,
        ("C", "Y") => 2,
        ("C", "Z") => 6,
        (_t,_m) => unreachable!()
    }
}


fn part_two<I>(input:I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut running_sum = 0;
    for round in input {
        if !round.is_empty() {
            let split = round.split(' ').take(2).collect::<Vec<&str>>();
            let their_move = split[0];
            let my_move = match split[1]  {
                // Loss
                "X" => {
                    match their_move {
                        "A" => "Z",
                        "B" => "X",
                        "C" => "Y",
                        _ => unreachable!()
                    }
                },
                // Draw
                "Y" => match their_move {
                        "A" => "X",
                        "B" => "Y",
                        "C" => "Z",
                        _ => unreachable!()
                    },
                // Win
                "Z" => match their_move {
                        "A" => "Y",
                        "B" => "Z",
                        "C" => "X",
                        _ => unreachable!()
                    },
                _ => unreachable!()
            };
            running_sum += run_match(their_move, my_move);
        }
    }
    Ok(running_sum)
}


#[cfg(test)]
mod tests {
    use crate::day_two::{part_two, part_two_file};

    use super::{part_one, part_one_file};


    #[test]
    fn test_part_one_example() {
        let input = "A Y\nB X\nC Z";
        let result = part_one(input.split('\n').map(|x|x.to_string()));
        assert!(result.is_ok());
        assert_eq!(15, result.unwrap());
    }

    #[test]
    fn test_part_one_my_input() {
        let result = part_one_file("src/day_two/input.txt");
        if result.is_err() {
            panic!("Expected Ok Got {:?}", result);
        }
        assert_eq!(15337, result.unwrap());
    }


    #[test]
    fn test_part_two_example() {
        let input = "A Y\nB X\nC Z";
        let result = part_two(input.split('\n').map(|x|x.to_string()));
        assert!(result.is_ok());
        assert_eq!(12, result.unwrap());
    }

    #[test]
    fn test_part_two_my_input() {
        let result = part_two_file("src/day_two/input.txt");
        if result.is_err() {
            panic!("Expected Ok Got {:?}", result);
        }
        assert_eq!(11696, result.unwrap());
    }
}
