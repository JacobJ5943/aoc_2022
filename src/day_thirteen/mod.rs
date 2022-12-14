use std::cmp::Ordering;

use color_eyre::{Report, Result};
use either::Either;
use nom::{branch::alt, bytes::complete::tag_no_case, character::complete::digit1, IResult};

fn part_two<I>(input: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut parsed_lines = Vec::new();
    for line in input.filter(|s| !s.is_empty()) {
        parsed_lines.push(parse_packet(&line)?.0);
    }
    parsed_lines.push(parse_packet("[[2]]")?.0);
    parsed_lines.push(parse_packet("[[6]]")?.0);
    parsed_lines.sort();

    let divisor_2 = parse_packet("[[2]]")?.0;
    let divisor_6 = parse_packet("[[6]]")?.0;
    let mut final_result = 1;
    for (parsed_line, index) in parsed_lines.iter().zip(1..) {
        if parsed_line == &divisor_2 || parsed_line == &divisor_6 {
            final_result *= index;
        }
    }
    Ok(final_result)
}

fn part_one<I>(mut input: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let mut indices = Vec::new();
    let mut index = 0;
    let mut sum = 0;
    while let (Some(top), Some(bottom), _) = (input.next(), input.next(), input.next()) {
        index += 1;
        
        indices.push(index);
        let parsed_top = parse_packet(&top)?;

        let parsed_bottom = parse_packet(&bottom)?;

        if let PacketOrderingResult::Equal | PacketOrderingResult::InOrder =
            are_packets_in_order(&parsed_top.0, &parsed_bottom.0)?
        {
            sum += index;
        }
    }
    Ok(sum)
}

#[derive(Eq, PartialEq, Debug)]
enum PacketOrderingResult {
    OutOfOrder,
    InOrder,
    Equal,
}

fn are_packets_in_order(left: &Packet, right: &Packet) -> Result<PacketOrderingResult> {
    for (index, right_value) in right.inner.iter().enumerate() {
        if let Some(left_value) = left.inner.get(index) {
            match (left_value, right_value) {
                (Either::Left(left), Either::Left(right)) => {
                    let result = are_packets_in_order(left, right)?;
                    match result {
                        PacketOrderingResult::OutOfOrder => {
                            return Ok(PacketOrderingResult::OutOfOrder)
                        }
                        PacketOrderingResult::InOrder => return Ok(PacketOrderingResult::InOrder),
                        PacketOrderingResult::Equal => (), // Continue to the next item
                    }
                }
                (Either::Left(left_packet), Either::Right(right_value)) => {
                    match are_packets_in_order(
                        left_packet,
                        &Packet {
                            inner: vec![Either::Right(*right_value)],
                        },
                    )? {
                        PacketOrderingResult::OutOfOrder => {
                            return Ok(PacketOrderingResult::OutOfOrder)
                        }
                        PacketOrderingResult::InOrder => return Ok(PacketOrderingResult::InOrder),
                        PacketOrderingResult::Equal => (),
                    }
                }
                (Either::Right(left_value), Either::Left(right_packet)) => {
                    match (are_packets_in_order(
                        &Packet {
                            inner: vec![Either::Right(*left_value)],
                        },
                        right_packet,
                    ))? {
                        PacketOrderingResult::OutOfOrder => {
                            return Ok(PacketOrderingResult::OutOfOrder)
                        }
                        PacketOrderingResult::InOrder => return Ok(PacketOrderingResult::InOrder),
                        PacketOrderingResult::Equal => (),
                    }
                }
                (Either::Right(left_value), Either::Right(right_value)) => {
                    match left_value.cmp(right_value) {
                        std::cmp::Ordering::Less => return Ok(PacketOrderingResult::InOrder),
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => return Ok(PacketOrderingResult::OutOfOrder),
                    }
                }
            }
        }
    }

    Ok(match left.inner.len().cmp(&right.inner.len()) {
        std::cmp::Ordering::Less => PacketOrderingResult::InOrder,
        std::cmp::Ordering::Equal => PacketOrderingResult::Equal,
        std::cmp::Ordering::Greater => PacketOrderingResult::OutOfOrder,
    })
}

#[derive(Clone, Debug)]
struct Packet {
    inner: Vec<Either<Packet, usize>>,
}

impl Eq for Packet {}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            are_packets_in_order(self, other).unwrap(),
            PacketOrderingResult::Equal
        )
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match are_packets_in_order(self, other).unwrap() {
            PacketOrderingResult::OutOfOrder => Some(Ordering::Greater),
            PacketOrderingResult::InOrder => Some(Ordering::Less),
            PacketOrderingResult::Equal => Some(Ordering::Equal),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match are_packets_in_order(self, other).unwrap() {
            PacketOrderingResult::OutOfOrder => std::cmp::Ordering::Greater,
            PacketOrderingResult::InOrder => std::cmp::Ordering::Less,
            PacketOrderingResult::Equal => std::cmp::Ordering::Equal,
        }
    }
}

fn parse_packet(input: &str) -> Result<(Packet, &str)> {
    let mut inner = Vec::new();

    // First see if I encounter a list or a value
    let mut working_string = input;
    while !working_string.is_empty() {
        let separator: IResult<&str, &str> = tag_no_case(",")(working_string);
        if let Ok(rest) = separator {
            working_string = rest.0;
        }

        let result: IResult<&str, &str> = alt((tag_no_case("["), tag_no_case("]")))(working_string);
        match result {
            Ok(good_result) => match good_result.1 {
                // THis may be wrong and suppose to be 1
                "[" => {
                    let deeper = parse_packet(good_result.0)?;
                    inner.push(Either::Left(deeper.0));
                    working_string = deeper.1;
                } // go deeper
                "]" => {
                    return Ok((Packet { inner }, good_result.0));
                } // return me now
                _ => unreachable!("This should be unreachable as the parser succeeded"),
            },
            Err(_err) => {
                let result: IResult<&str, &str> = digit1(working_string);
                match result {
                    Ok(good_result) => {
                        inner.push(Either::Right(good_result.1.parse::<usize>()?));
                        working_string = good_result.0
                    }
                    Err(err) => return Err(Report::msg(format!("Failed with error {:?}", err))),
                }
            }
        }
    }

    Ok((Packet { inner }, working_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        let result = parse_packet(",[1]").unwrap();
        assert_eq!(
            &result.0,
            &Packet {
                inner: vec![Either::Left(Packet {
                    inner: vec![Either::Right(1)]
                })]
            }
        );

        let result = parse_packet(",[1,[2]]").unwrap();
        assert_eq!(
            &result.0,
            &Packet {
                inner: vec![Either::Left(Packet {
                    inner: vec![
                        Either::Right(1),
                        Either::Left(Packet {
                            inner: vec![Either::Right(2)]
                        })
                    ]
                })]
            }
        );

        let result = parse_packet(",[1,[2],3]").unwrap();
        assert_eq!(
            &result.0,
            &Packet {
                inner: vec![Either::Left(Packet {
                    inner: vec![
                        Either::Right(1),
                        Either::Left(Packet {
                            inner: vec![Either::Right(2)]
                        }),
                        Either::Right(3)
                    ]
                })]
            }
        )
    }

    #[test]
    fn test_are_packets_in_order() {
        let (left, _) = parse_packet("[1,1,3,1,1]").unwrap();
        let (right, _) = parse_packet("[1,1,5,1,1]").unwrap();
        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::InOrder
        );

        let (left, _) = parse_packet("[9]").unwrap();
        let (right, _) = parse_packet("[[8,7,6]]").unwrap();

        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::OutOfOrder
        );

        let (left, _) = parse_packet("[[1],[2,3,4]]").unwrap();
        let (right, _) = parse_packet("[[1],4]").unwrap();

        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::InOrder
        );

        let (left, _) = parse_packet("[8]").unwrap();
        let (right, _) = parse_packet("[[8,7,6]]").unwrap();

        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::InOrder
        );

        let (left, _) = parse_packet("[[8,7,6]]").unwrap();
        let (right, _) = parse_packet("[8]").unwrap();

        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::OutOfOrder
        );

        let (left, _) = parse_packet("[1]").unwrap();
        let (right, _) = parse_packet("[[1]]").unwrap();

        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::Equal
        );

        let (left, _) = parse_packet("[[[]]]").unwrap();
        let (right, _) = parse_packet("[[]]").unwrap();

        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::OutOfOrder
        );
        let (left, _) = parse_packet("[[]]").unwrap();
        let (right, _) = parse_packet("[[[]]]").unwrap();

        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::InOrder
        );
        let (left, _) = parse_packet("[3]").unwrap();
        let (right, _) = parse_packet("[3,3]").unwrap();
        let (left, _) = parse_packet("[3,3]").unwrap();
        let (right, _) = parse_packet("[3]").unwrap();

        assert_eq!(
            are_packets_in_order(&left, &right).unwrap(),
            PacketOrderingResult::OutOfOrder
        );
    }

    #[test]
    fn test_part_one_example() {
        let input = vec![
            "[1,1,3,1,1]".to_string(),
            "[1,1,5,1,1]".to_string(),
            "".to_string(),
            "[[1],[2,3,4]]".to_string(),
            "[[1],4]".to_string(),
            "".to_string(),
            "[9]".to_string(),
            "[[8,7,6]]".to_string(),
            "".to_string(),
            "[[4,4],4,4]".to_string(),
            "[[4,4],4,4,4]".to_string(),
            "".to_string(),
            "[7,7,7,7]".to_string(),
            "[7,7,7]".to_string(),
            "".to_string(),
            "[]".to_string(),
            "[3]".to_string(),
            "".to_string(),
            "[[[]]]".to_string(),
            "[[]]".to_string(),
            "".to_string(),
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string(),
            "".to_string(),
        ];

        assert_eq!(part_one(input.into_iter()).unwrap(), 13);
    }

    #[test]
    fn test_part_one_my_input() {
        assert_eq!(
            part_one(crate::read_file_line_by_line("src/day_thirteen/input.txt").unwrap()).unwrap(),
            6428
        );
    }

    #[test]
    fn test_part_two_example() {
        let input = vec![
            "[1,1,3,1,1]".to_string(),
            "[1,1,5,1,1]".to_string(),
            "".to_string(),
            "[[1],[2,3,4]]".to_string(),
            "[[1],4]".to_string(),
            "".to_string(),
            "[9]".to_string(),
            "[[8,7,6]]".to_string(),
            "".to_string(),
            "[[4,4],4,4]".to_string(),
            "[[4,4],4,4,4]".to_string(),
            "".to_string(),
            "[7,7,7,7]".to_string(),
            "[7,7,7]".to_string(),
            "".to_string(),
            "[]".to_string(),
            "[3]".to_string(),
            "".to_string(),
            "[[[]]]".to_string(),
            "[[]]".to_string(),
            "".to_string(),
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string(),
            "".to_string(),
        ];

        assert_eq!(part_two(input.into_iter()).unwrap(), 140);
    }

    #[test]
    fn test_part_two_my_input() {
        assert_eq!(
            part_two(crate::read_file_line_by_line("src/day_thirteen/input.txt").unwrap()).unwrap(),
            22464
        );
    }
}
