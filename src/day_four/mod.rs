use color_eyre::{Report, Result};

struct Elf {
    small: usize,
    high: usize,
}

fn part_two_from_iter<I>(input_lines: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut overlap_count = 0;

    for line in input_lines.map(|x| x.trim_end().trim_start().to_string()) {
        let mut line_split = line.split(',');
        match (line_split.next(), line_split.next(), line_split.next()) {
            (Some(first_elf), Some(second_elf), None) => {
                let mut iter = first_elf.split('-');
                let first_elf = Elf {
                    small: iter
                        .next()
                        .ok_or_else(|| Report::msg("Failed to find low range"))?
                        .parse()?,
                    high: iter
                        .next()
                        .ok_or_else(|| Report::msg("Failed to find high range"))?
                        .parse()?,
                };

                let mut iter = second_elf.split('-');
                let second_elf = Elf {
                    small: iter
                        .next()
                        .ok_or_else(|| Report::msg("Failed to find low range"))?
                        .parse()?,
                    high: iter
                        .next()
                        .ok_or_else(|| Report::msg("Failed to find high range"))?
                        .parse()?,
                };

                match first_elf.small.cmp(&second_elf.small) {
                    std::cmp::Ordering::Less => {
                        if first_elf.high >= second_elf.small {
                            overlap_count += 1
                        }
                    }
                    std::cmp::Ordering::Equal => overlap_count += 1, // This means that one has to be a subset of the other,
                    std::cmp::Ordering::Greater => {
                        if second_elf.high >= first_elf.small {
                            overlap_count += 1;
                        }
                    }
                }
            }
            _ => {
                return Err(Report::msg(
                    "the number of elves found was not equal to the required 2",
                ))
            }
        }
    }
    Ok(overlap_count)
}

fn part_one_from_iter<I>(input_lines: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut complete_overlap_count = 0;

    for line in input_lines.map(|x| x.trim_end().trim_start().to_string()) {
        let mut line_split = line.split(',');
        match (line_split.next(), line_split.next(), line_split.next()) {
            (Some(first_elf), Some(second_elf), None) => {
                let mut iter = first_elf.split('-');
                let first_elf = Elf {
                    small: iter
                        .next()
                        .ok_or_else(|| Report::msg("Failed to find low range"))?
                        .parse()?,
                    high: iter
                        .next()
                        .ok_or_else(|| Report::msg("Failed to find high range"))?
                        .parse()?,
                };

                let mut iter = second_elf.split('-');
                let second_elf = Elf {
                    small: iter
                        .next()
                        .ok_or_else(|| Report::msg("Failed to find low range"))?
                        .parse()?,
                    high: iter
                        .next()
                        .ok_or_else(|| Report::msg("Failed to find high range"))?
                        .parse()?,
                };

                match first_elf.small.cmp(&second_elf.small) {
                    std::cmp::Ordering::Less => {
                        if first_elf.high >= second_elf.high {
                            complete_overlap_count += 1
                        }
                    }
                    std::cmp::Ordering::Equal => complete_overlap_count += 1, // This means that one has to be a subset of the other,
                    std::cmp::Ordering::Greater => {
                        if second_elf.high >= first_elf.high {
                            complete_overlap_count += 1;
                        }
                    }
                }
            }
            _ => {
                return Err(Report::msg(
                    "the number of elves found was not equal to the required 2",
                ))
            }
        }
    }
    Ok(complete_overlap_count)
}

#[cfg(test)]
mod tests {
    use crate::read_file_line_by_line;

    use super::part_one_from_iter;
    use super::part_two_from_iter;

    #[test]
    fn test_part_one_example() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .split('\n').map(str::to_string);

        let result = part_one_from_iter(input);
        match result {
            Ok(result) => assert_eq!(result, 2),
            Err(err) => panic!("Expected Ok got Err({:?})", err),
        }
    }

    #[test]
    fn test_part_one_my_input() {
        let input = read_file_line_by_line("src/day_four/input.txt").unwrap();

        let result = part_one_from_iter(input);
        match result {
            Ok(result) => assert_eq!(result, 444),
            Err(err) => panic!("Expected Ok got Err({:?})", err),
        }
    }


    #[test]
    fn test_part_two_example() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .split('\n').map(str::to_string);

        let result = part_two_from_iter(input);
        match result {
            Ok(result) => assert_eq!(result, 4),
            Err(err) => panic!("Expected Ok got Err({:?})", err),
        }
    }

#[test]
    fn test_part_two_my_input() {
        let input = read_file_line_by_line("src/day_four/input.txt").unwrap();

        let result = part_two_from_iter(input);
        match result {
            Ok(result) => assert_eq!(result, 801),
            Err(err) => panic!("Expected Ok got Err({:?})", err),
        }
    }
}
