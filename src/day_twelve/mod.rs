#![allow(dead_code)]
use std::ops::Deref;

use color_eyre::{Report, Result};
use petgraph::{
    adj::NodeIndex, algo::astar, prelude::DiGraph, stable_graph::IndexType,
    visit::IntoNodeReferences, Directed, Graph,
};

/// The node location is going to be (the number of items per column * row_index) + col_index
struct NodeLocation(usize);

impl Deref for NodeLocation {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
type MyNodeEdge = (usize, usize, u32);

fn create_graph_from_input_iterator<I>(
    input: I,
) -> Result<(
    Graph<usize, usize, Directed, usize>,
    (usize, usize),
    (usize, usize),
    usize,
)>
where
    I: Iterator<Item = String>,
{
    let mut graph_vec = Vec::new();

    let mut start_location = None;
    let mut end_location = None;
    // This could be done in one pass by processing a window of 3 rows.
    // This would give a top, middle, and bottom for each location in the middle row.  Which is what is needed to calculate the edges
    // Instead I'm adding all of the nodes to a vec in memory.  That way I can then do a second pass and check each neighbor
    for (row_index, line) in input.enumerate() {
        graph_vec.push(Vec::new());
        for (col_index, mut node_value) in line.chars().enumerate() {
            match node_value {
                'S' => {
                    start_location = Some((row_index, col_index));
                    node_value = 'a';
                }
                'E' => {
                    end_location = Some((row_index, col_index));
                    node_value = 'z';
                }
                _ => (),
            }
            // graph_vec.last_mut().unwrap().push(node_value.to_digit(10).ok_or_else(||Report::msg(format!("Failed to convert char {} to digit", node_value)))?);
            graph_vec.last_mut().unwrap().push(node_value as u32);
        }
    }

    // Now for each node in graph_vec
    // Check each neighbor and create it's edges
    let mut edges = Vec::new();
    let number_of_items_per_row = graph_vec
        .get(0)
        .ok_or_else(|| Report::msg("graph_vec had no rows"))?
        .len();
    for (row_index, row) in graph_vec.iter().enumerate() {
        for (col_index, node_value) in row.iter().enumerate() {
            let mut x = get_neighbors(&graph_vec, row_index, col_index)?
                .into_iter()
                .filter(|node_edge| node_edge.2 <= (node_value + 1))
                .map(|new_end_point| {
                    (
                        row_index * number_of_items_per_row + col_index,
                        new_end_point.0 * number_of_items_per_row + new_end_point.1,
                        1,
                    )
                })
                .collect::<Vec<(usize, usize, usize)>>();

            edges.append(&mut x);
        }
    }

    let mut graph: Graph<usize, usize, Directed, usize> = DiGraph::from_edges(edges);
    // Now add the node weights
    // A step only needed in part two because I'm not keeping track of all the 'a' locations

    for (row_index, row) in graph_vec.iter().enumerate() {
        for (col_index, node_value) in row.iter().enumerate() {
            let x = graph
                .node_weight_mut(NodeIndex::from(
                    row_index * number_of_items_per_row + col_index,
                ))
                .ok_or_else(|| Report::msg("Error adding node weights"))?;
            *x = *node_value as usize;
        }
    }

    // find start_index
    let start_location =
        start_location.ok_or_else(|| Report::msg("Failed to find start location"))?;
    let end_location = end_location.ok_or_else(|| Report::msg("Failed to find end location"))?;

    Ok((graph, start_location, end_location, number_of_items_per_row))
}

fn part_one<I>(input: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let (graph, start_location, end_location, number_of_items_per_row) =
        create_graph_from_input_iterator(input)?;

    let start_location =
        NodeIndex::from(start_location.0 * number_of_items_per_row + start_location.1);

    let end_location: NodeIndex<usize> = end_location.0 * number_of_items_per_row + end_location.1;

    let result = astar(
        &graph,
        start_location,
        |checking_node| checking_node.index() == end_location.index(),
        |_e| 1,
        |_e| 0,
    );
    Ok(result
        .ok_or_else(|| Report::msg("Failed to find path from start to end"))?
        .0)
}

fn part_two<I>(input: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    let (graph, _start_location, end_location, number_of_items_per_row) =
        create_graph_from_input_iterator(input)?;

    // Could probably bellman ford it with the start_location actually being the end location.  I would need to change all of the edges though to be reversed
    // i'm just failed so miserably at setting up the graph and I don't want to deal with that right now
    let end_location: NodeIndex<usize> = end_location.0 * number_of_items_per_row + end_location.1;

    let mut max_steps = usize::MAX;
    for start_location in graph
        .node_references()
        .filter(|node| node.1 == &('a' as u32 as usize))
    {
        if let Some(result) = astar(
            &graph,
            start_location.0,
            |checking_node| checking_node.index() == end_location.index(),
            |_e| 1,
            |_e| 0,
        ) {
            max_steps = max_steps.min(result.0);
        } else {
            println!(
                "Failed to find path from start location {:?} to end_location {:?}",
                start_location, end_location
            );
        }
    }

    Ok(max_steps)
}
fn get_neighbors(
    graph: &Vec<Vec<u32>>,
    row_index: usize,
    col_index: usize,
) -> Result<Vec<MyNodeEdge>> {
    let mut neighbors: Vec<MyNodeEdge> = Vec::new();
    let col_max_length = graph
        .get(0)
        .ok_or_else(|| Report::msg("no rows in graph"))?
        .len();

    if row_index > 0 {
        neighbors.push((
            row_index - 1,
            col_index,
            *graph
                .get(row_index - 1)
                .ok_or_else(|| Report::msg("row_index checked out of bounds"))?
                .get(col_index)
                .ok_or_else(|| Report::msg("col_index checked out of bounds"))?,
        ));
    }
    if row_index < graph.len() - 1 {
        neighbors.push((
            row_index + 1,
            col_index,
            *graph
                .get(row_index + 1)
                .ok_or_else(|| Report::msg("row_index checked out of bounds"))?
                .get(col_index)
                .ok_or_else(|| Report::msg("col_index checked out of bounds"))?,
        ));
    }

    if col_index < col_max_length - 1 {
        neighbors.push((
            row_index,
            col_index + 1,
            *graph
                .get(row_index)
                .ok_or_else(|| Report::msg("row_index checked out of bounds"))?
                .get(col_index + 1)
                .ok_or_else(|| Report::msg("col_index checked out of bounds"))?,
        ));
    }

    if col_index > 0 {
        neighbors.push((
            row_index,
            col_index - 1,
            *graph
                .get(row_index)
                .ok_or_else(|| Report::msg("row_index checked out of bounds"))?
                .get(col_index - 1)
                .ok_or_else(|| Report::msg("col_index checked out of bounds"))?,
        ));
    }
    Ok(neighbors)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one_example() {
        let input = vec![
            "Sabqponm".to_string(),
            "abcryxxl".to_string(),
            "accszExk".to_string(),
            "acctuvwj".to_string(),
            "abdefghi".to_string(),
        ];

        assert_eq!(part_one(input.into_iter()).unwrap(), 31);
    }

    #[test]
    fn test_part_one_my_input() {
        assert_eq!(
            part_one(crate::read_file_line_by_line("src/day_twelve/input.txt").unwrap()).unwrap(),
            339
        );
    }

    #[test]
    fn test_part_two_example() {
        let input = vec![
            "Sabqponm".to_string(),
            "abcryxxl".to_string(),
            "accszExk".to_string(),
            "acctuvwj".to_string(),
            "abdefghi".to_string(),
        ];

        assert_eq!(part_two(input.into_iter()).unwrap(), 29);
    }

    #[test]
    fn test_part_two_my_input() {
        assert_eq!(
            part_two(crate::read_file_line_by_line("src/day_twelve/input.txt").unwrap()).unwrap(),
            339
        );
    }
}
