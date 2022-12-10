#![allow(dead_code, unused)]
use color_eyre::{Report, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    multi::many0,
    sequence::terminated,
    IResult,
};
use regex::Regex;

fn new_working_dir(current_dir: &str, new_dir: &str) -> String {
    let mut split_iter = new_dir.trim_end_matches('/').split('/');

    let mut current_directory = current_dir
        .trim_end_matches('/')
        .split('/')
        .collect::<Vec<&str>>(); // Oh I hate how much collecting I need to do since this is every iterator
    match &mut split_iter.next() {
        Some("") => {
            current_directory.clear();
            current_directory.push("")
        }
        Some(".") => (), // current iter stay the say
        Some("..") => {
            current_directory.pop();
        }
        Some(new_base_dir) => {
            current_directory.push(new_base_dir);
        }
        None => panic!("hah"),
    }

    for next_dir in split_iter {
        match next_dir {
            "" => (),
            "." => (), // current iter stay the say
            ".." => {
                current_directory.pop();
            }
            new_base_dir => current_directory.push(new_base_dir),
        }
    }

    //"/".to_string() + &current_directory.join("/")
    if current_directory.len() == 1 && current_directory[0].is_empty() {
        "/".to_string()
    } else {
        current_directory.join("/")
    }
}

#[derive(Default, Debug)]
struct Directory {
    path: String,
    other_dirs: Vec<Rc<RefCell<Directory>>>,
    files: Vec<(usize, String)>,
}

fn part_one<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    let filesystem_map = create_filesystem_part_one(input).unwrap();

    let ((_root_path, _root_size), dfs_result) =
        dfs_directory_size(filesystem_map.get("/").unwrap());
    dfs_result
        .into_iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum::<usize>()
}

fn create_filesystem_part_one<I>(input: I) -> Result<HashMap<String, Rc<RefCell<Directory>>>>
where
    I: Iterator<Item = String>,
{
    let mut filesystem_map: HashMap<String, Rc<RefCell<Directory>>> = HashMap::new();

    let mut current_directory = "/".to_string();

    let cd_regex = Regex::new("\\$ cd (.+)")?;
    let ls_regex = Regex::new("\\$ ls")?;
    let file_regex = Regex::new("([0-9]+) (.*)")?;
    let dir_regex = Regex::new("dir (.*)")?;

    for line in input {
        if let Some(captures) = cd_regex.captures(&line) {
            let new_dir = captures
                .get(1)
                .ok_or_else(|| Report::msg("Failed to get directory for cd command"))?
                .as_str();
            current_directory = new_working_dir(current_directory.as_str(), new_dir);
            if !filesystem_map.contains_key(&current_directory) {
                filesystem_map.insert(
                    current_directory.clone(),
                    Rc::new(RefCell::new(Directory {
                        path: current_directory.clone(),
                        ..Default::default()
                    })),
                );
            }
        } else if ls_regex.is_match(&line) {
            // This will then read line by line anyway sooooo w.e
        } else if let Some(captures) = file_regex.captures(&line) {
            let file_size = captures
                .get(1)
                .ok_or_else(|| Report::msg("Failed to get file_size"))?
                .as_str()
                .parse::<usize>()?;
            let file_name = captures
                .get(2)
                .ok_or_else(|| Report::msg("Failed to get filename"))?
                .as_str();
            let directory = filesystem_map.get_mut(&current_directory).unwrap();
            directory
                .borrow_mut()
                .files
                .push((file_size, file_name.to_string()));
        } else if let Some(captures) = dir_regex.captures(&line) {
            let sub_dir = captures
                .get(1)
                .ok_or_else(|| Report::msg("Failed to get subdir name"))?
                .as_str();
            let sub_dir = match current_directory.as_str() {
                "/" => format!("/{}", sub_dir),
                _ => format!("{}/{}", &current_directory, sub_dir),
            };

            let sub_dir_rc = match filesystem_map.get(&sub_dir) {
                Some(sub_dir_rc) => sub_dir_rc.clone(),
                None => {
                    filesystem_map.insert(
                        sub_dir.to_string(),
                        Rc::new(RefCell::new(Directory {
                            path: sub_dir.to_string(),
                            ..Default::default()
                        })),
                    );
                    filesystem_map.get(&sub_dir).unwrap().clone()
                }
            };

            filesystem_map
                .get_mut(current_directory.as_str())
                .unwrap()
                .borrow_mut()
                .other_dirs
                .push(sub_dir_rc.clone());
        } else {
            unimplemented!()
        }
    }

    Ok(filesystem_map)
}

fn part_two<I>(input: I) -> usize
where
    I: Iterator<Item = String>,
{
    let filesystem_map = create_filesystem_part_one(input).unwrap();

    let ((_root_path, root_size), all_dir_sizes) =
        dfs_directory_size(filesystem_map.get("/").unwrap());
    // Could have changed to only do one pass, but it's late
    let space_to_clear = root_size - (70000000 - 30000000);
    let mut min = usize::MAX;

    for (_path_name, size) in all_dir_sizes {
        if size >= space_to_clear {
            min = min.min(size);
        }
    }
    if min == usize::MAX {
        panic!("failed")
    } else {
        min
    }
}

fn dfs_directory_size(
    current_dir: &Rc<RefCell<Directory>>,
) -> ((String, usize), Vec<(String, usize)>) {
    let file_sizes: usize = current_dir.borrow().files.iter().map(|x| x.0).sum();
    // This assumes that there are no cycles (symlinks).
    // If there are cycles then this will stack overflow
    let sub_dir_results = current_dir
        .borrow()
        .other_dirs
        .iter()
        .map(dfs_directory_size)
        .collect::<Vec<_>>();

    let mut running_sub_dir_size = 0;
    let mut all_sub_dirs = Vec::new();
    for ((sub_dir_path, sub_dir_size), mut all_sub_dir_sizes) in sub_dir_results {
        running_sub_dir_size += sub_dir_size;
        all_sub_dirs.append(&mut all_sub_dir_sizes);
        all_sub_dirs.push((sub_dir_path, sub_dir_size));
    }
    let current_dir_size = file_sizes + running_sub_dir_size;

    (
        (current_dir.borrow().path.to_string(), current_dir_size),
        all_sub_dirs,
    )
}

#[cfg(test)]
mod tests {

    use nom::character::complete::one_of;
    use nom::error::context;
    use nom::sequence::tuple;

    use super::new_working_dir;
    use super::*;

    #[test]
    fn test_charing_dirs() {
        assert_eq!(
            new_working_dir("/home/jacob/blah/blahblah/", "/home/"),
            "/home".to_string()
        );
        assert_eq!(
            new_working_dir("/home/jacob/blah/blahblah/", "./"),
            "/home/jacob/blah/blahblah".to_string()
        );
        assert_eq!(
            new_working_dir("/home/jacob/blah/blahblah/", "../../"),
            "/home/jacob".to_string()
        );
        assert_eq!(
            new_working_dir("/home/jacob/blah/blahblah/", "../.././"),
            "/home/jacob".to_string()
        );
        assert_eq!(new_working_dir("/", "./"), "/".to_string());
        assert_eq!(new_working_dir("/", "blah"), "/blah".to_string());
        assert_eq!(new_working_dir("/foo", "blah"), "/foo/blah".to_string());
    }

    #[test]
    fn test_part_one_example() {
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];

        let result = part_one(input.into_iter());
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part_one_my_input() {
        let result = part_one(crate::read_file_line_by_line("src/day_seven/input.txt").unwrap());
        assert_eq!(result, 1543140);
    }

    #[test]
    fn test_part_two_example() {
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];

        let result = part_two(input.into_iter());
        assert_eq!(result, 24933642);
    }

    #[test]
    fn test_part_two_my_input() {
        let result = part_two(crate::read_file_line_by_line("src/day_seven/input.txt").unwrap());
        assert_eq!(result, 1117448)
    }
}
