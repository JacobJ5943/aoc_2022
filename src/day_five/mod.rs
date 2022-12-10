#![allow(dead_code, unused)]
use color_eyre::{Report, Result};
use regex::Regex;

fn parse_input_initial_state<I>(mut input_iterator: I) -> Result<(Vec<Vec<char>>, I)>
where
    I: Iterator<Item = String>,
{
    let mut graph_lines = Vec::new();
    while let Some(line) = input_iterator.by_ref().next() {
        let trimmed_line = line.trim_end_matches('\n');
        if trimmed_line.is_empty() {
            break;
        } else {
            graph_lines.push(line);
        }
    }

    // I'm just going to use the input I'm given as gospel for parsing
    // using other things than spaces likes tabs is a problem for another day if ever

    let mut final_vec = vec![
        Vec::new();
        (graph_lines
            .get(0)
            .ok_or_else(|| color_eyre::Report::msg("No lines found for the initial setup state"))?
            .len()
            + 1)
            / 4
    ]; // + 1 since there will be one less space for the separators

    for line in graph_lines.into_iter().rev() {
        // rev so the top crate is pushed last

        for (final_index, index_in_line) in (1..line.len()).step_by(4).enumerate() {
            let char_to_insert = line.chars().nth(index_in_line).ok_or_else(|| {
                color_eyre::Report::msg(format!("expected_char in column {}", index_in_line / 4))
            })?;
            match &char_to_insert {
                ' ' => (),
                _ => final_vec.get_mut(final_index).unwrap().push(char_to_insert),
            }
        }
    }
    Ok((final_vec, input_iterator))
}

fn part_one_follow_the_rules<I>(input_iterator: I) -> Result<String>
where
    I: Iterator<Item = String>,
{
    let (mut graph, input_iterator) = parse_input_initial_state(input_iterator)?;
    let move_regex = Regex::new("move ([^ ]*) from ([^ ]*) to ([^ ]*)")?;
    for line in input_iterator {
        let captures = move_regex.captures(&line).ok_or_else(|| {
            Report::msg(format!("failed to find move locations in line->{:?}", line))
        })?;
        if captures.len() != 4 {
            return Err(Report::msg(format!(
                "unexpected number of captures for line->{:?}",
                line
            )));
        }
        for _ in 0..captures.get(1).unwrap().as_str().parse::<usize>()? {
            let expected_index = captures.get(2).unwrap().as_str().parse::<usize>()? - 1;
            let container = graph
                .get_mut(expected_index)
                .ok_or_else(|| Report::msg("no column found at expected index {expected_index}"))?
                .pop()
                .ok_or_else(|| Report::msg("No container found at expected location"))?;
            graph
                .get_mut(captures.get(3).unwrap().as_str().parse::<usize>()? - 1)
                .ok_or_else(|| Report::msg("no column found at expected index {expected_index}"))?
                .push(container);
        }
    }

    let mut all_tops = "".to_string();
    for top in graph.into_iter().filter_map(|mut column| column.pop()) {
        all_tops += &top.to_string();
    }
    Ok(all_tops)
}

fn part_two_follow_the_rules<I>(input_iterator: I) -> Result<String>
where
    I: Iterator<Item = String>,
{
    let (mut graph, input_iterator) = parse_input_initial_state(input_iterator)?;
    let move_regex = Regex::new("move ([^ ]*) from ([^ ]*) to ([^ ]*)")?;
    for line in input_iterator {
        let captures = move_regex.captures(&line).ok_or_else(|| {
            Report::msg(format!("failed to find move locations in line->{:?}", line))
        })?;
        if captures.len() != 4 {
            return Err(Report::msg(format!(
                "unexpected number of captures for line->{:?}",
                line
            )));
        }

        let containers_to_grab = captures.get(1).unwrap().as_str().parse::<usize>()?;
        let expected_index = captures.get(2).unwrap().as_str().parse::<usize>()? - 1;
        let mut containers = Vec::with_capacity(containers_to_grab);
        for _ in 0..containers_to_grab {
            containers.push(
                graph
                    .get_mut(expected_index)
                    .ok_or_else(|| {
                        Report::msg("no column found at expected index {expected_index}")
                    })?
                    .pop()
                    .ok_or_else(|| Report::msg("No container found at expected location"))?,
            );
        }

        for container in containers.into_iter().rev() {
            graph
                .get_mut(captures.get(3).unwrap().as_str().parse::<usize>()? - 1)
                .ok_or_else(|| Report::msg("no column found at expected index {expected_index}"))?
                .push(container);
        }
    }

    let mut all_tops = "".to_string();
    for top in graph.into_iter().filter_map(|mut column| column.pop()) {
        all_tops += &top.to_string();
    }
    Ok(all_tops)
}

#[cfg(test)]
mod tests {
    use crate::read_file_line_by_line;

    use super::*;

    #[test]
    fn it_works() {}

    #[test]
    fn test_parse_input_initial_state() {
        let input_lines = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            "1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let (graph, iterator) =
            parse_input_initial_state(input_lines.into_iter().map(str::to_string)).unwrap();
        assert_eq!(graph, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
    }

    #[test]
    fn test_move_regex() {
        let move_regex = Regex::new("move ([^ ]?) from ([^ ]?) to ([^ ]?)").unwrap();
        let testing_moves = vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        dbg!(move_regex.captures(testing_moves[0]));
    }

    #[test]
    fn test_part_one_example() {
        let input_lines = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            "1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let result = part_one_follow_the_rules(input_lines.into_iter().map(str::to_string));
        match result {
            Ok(result) => assert_eq!(result, "CMZ".to_string()),
            Err(err) => panic!("{:?}", err),
        }
    }

    #[test]
    fn test_part_one_my_input() {
        let lines = read_file_line_by_line("src/day_five/input.txt").unwrap();
        let result = part_one_follow_the_rules(lines);
        match result {
            Ok(result) => assert_eq!(result, "RTGWZTHLD".to_string()),
            Err(err) => panic!("{:?}", err),
        }
    }

    #[test]
    fn test_part_two_example() {
        let input_lines = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            "1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];
        let result = part_two_follow_the_rules(input_lines.into_iter().map(str::to_string));
        match result {
            Ok(result) => assert_eq!(result, "MCD".to_string()),
            Err(err) => panic!("{:?}", err),
        }
    }

    #[test]
    fn test_part_two_my_input() {
        let lines = read_file_line_by_line("src/day_five/input.txt").unwrap();
        let result = part_two_follow_the_rules(lines);
        match result {
            Ok(result) => assert_eq!(result, "STHGRZZFR".to_string()),
            Err(err) => panic!("{:?}", err),
        }
    }
}
