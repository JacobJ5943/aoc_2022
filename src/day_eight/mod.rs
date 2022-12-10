#![allow(dead_code, unused)]
use std::collections::HashSet;

use color_eyre::{Report, Result};

fn create_grid<I>(input: I) -> Result<Vec<Vec<u8>>>
where
    I: Iterator<Item = String>,
{
    let mut input_vec: Vec<Vec<u8>> = Vec::new();
    for line in input {
        let line = line
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as u8)
                    .ok_or_else(|| Report::msg("Failed to parse char {} to u32"))
            })
            .collect::<Result<Vec<u8>>>()?;

        input_vec.push(line)
    }
    Ok(input_vec)
}
fn part_one<I>(input: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    // Just pull it all into memory.  I have 32GB
    let grid = create_grid(input)?;

    // Doing this in two passes.  Once keeping track of left, right and top to down
    // The second pass will just be checking the view from the bottom

    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let mut tallest_tree_top: Vec<u8> = Vec::with_capacity(
        grid.get(0)
            .ok_or_else(|| Report::msg("grid has no rows"))?
            .len(),
    );
    let mut tall_9_count = 0;

    for (row_index, row) in grid.iter().enumerate() {
        // check left
        let mut left_iter = row.iter().enumerate();
        let mut acc = left_iter
            .next()
            .ok_or_else(|| Report::msg("Row is empty"))?
            .1;
        visible_trees.insert((row_index, 0));
        for (column_index, tree) in left_iter {
            if tree > acc {
                visible_trees.insert((row_index, column_index));
                acc = tree;
                if *tree == 9 {
                    break;
                }
            }
        }

        // check right
        let mut right_iter = row.iter().enumerate().rev();
        let mut acc = right_iter
            .next()
            .ok_or_else(|| Report::msg("Row is empty"))?
            .1;
        visible_trees.insert((row_index, right_iter.len()));
        for (column_index, tree) in right_iter {
            if tree > acc {
                visible_trees.insert((row_index, column_index));
                acc = tree;
                if *tree == 9 {
                    break;
                }
            }
        }

        let col_count = row.len();
        // Check top
        for (column_index, tree) in row.iter().enumerate() {
            if tall_9_count == col_count {
                break;
            }
            if let Some(tallest) = tallest_tree_top.get(column_index) {
                if tallest < tree {
                    tallest_tree_top[column_index] = *tree;
                    visible_trees.insert((row_index, column_index));
                    if *tree == 9 {
                        tall_9_count += 1;
                    }
                }
            } else {
                if *tree == 9 {
                    tall_9_count += 1;
                }
                tallest_tree_top.push(*tree);
                visible_trees.insert((row_index, column_index));
            }
        }
    }

    // And now for another step bottom up.  It may be worth checking later if it's faster to have some way to keep track of bottom up in the first part.  as well as doing 1 full scan and 2 partial scans each row
    // Todo for another day
    // I think what I could do is have a Vec<Vec<u8>> where the inner vec is 10 spaces.  Each space is going to be the last index that had it.  So for each location throw it's value in that table.
    // After that the tallest trees could be calculated

    tallest_tree_top.clear();
    tall_9_count = 0;

    for (row_index, row) in grid.iter().enumerate().rev() {
        let col_count = row.len();
        // Check top
        for (column_index, tree) in row.iter().enumerate() {
            if tall_9_count == col_count {
                break;
            }
            if let Some(tallest) = tallest_tree_top.get(column_index) {
                if tallest < tree {
                    tallest_tree_top[column_index] = *tree;
                    visible_trees.insert((row_index, column_index));
                    if *tree == 9 {
                        tall_9_count += 1;
                    }
                }
            } else {
                if *tree == 9 {
                    tall_9_count += 1;
                }
                tallest_tree_top.push(*tree);
                visible_trees.insert((row_index, column_index));
            }
        }
    }

    Ok(visible_trees.len())
}

fn part_two<I>(input: I) -> Result<usize>
where
    I: Iterator<Item = String>,
{
    // Not sure how to speed this up so I'm just going to check each one
    let grid = create_grid(input)?;
    let col_count = grid
        .get(0)
        .ok_or_else(|| Report::msg("No items in grid"))?
        .len();
    let row_count = grid.len();
    let mut max_scenic_score = 0;
    for row_index in 0..row_count {
        for col_index in 0..col_count {
            let tree_height = grid[row_index][col_index];

            let mut trees_left = 0;
            let mut trees_right = 0;
            let mut trees_down = 0;
            let mut trees_up = 0;

            // Look left
            for checking_col_index in (0..col_index).rev() {
                if grid[row_index][checking_col_index] < tree_height {
                    trees_left += 1;
                } else {
                    trees_left += 1;
                    break;
                }
            }

            // Look right
            for checking_col_index in (col_index + 1)..col_count {
                if grid[row_index][checking_col_index] < tree_height {
                    trees_right += 1;
                } else {
                    trees_right += 1;
                    break;
                }
            }

            // Look down
            for checking_row_index in (row_index + 1)..row_count {
                if grid[checking_row_index][col_index] < tree_height {
                    trees_down += 1;
                } else {
                    trees_down += 1;
                    break;
                }
            }

            // Look up
            for checking_row_index in (0..row_index).rev() {
                if grid[checking_row_index][col_index] < tree_height {
                    trees_up += 1;
                } else {
                    trees_up += 1;
                    break;
                }
            }

            max_scenic_score =
                max_scenic_score.max(trees_left * trees_right * trees_down * trees_up);
        }
    }

    Ok(max_scenic_score)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let result = part_one(input.into_iter()).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_one_my_input() {
        let result =
            part_one(crate::read_file_line_by_line("src/day_eight/input.txt").unwrap()).unwrap();
        assert_eq!(result, 1794);
    }

    #[test]
    fn test_part_two_example() {
        let input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let result = part_two(input.into_iter()).unwrap();
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_two_my_input() {
        let result =
            part_two(crate::read_file_line_by_line("src/day_eight/input.txt").unwrap()).unwrap();
        assert_eq!(result, 199272);
    }
}
