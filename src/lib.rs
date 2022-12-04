use color_eyre::Result;
use std::io::BufRead;
use std::{io::BufReader, path::Path};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod day_one;
mod day_two;
mod day_three;
mod day_four;

fn read_file_line_by_line<P>(input_path: P) -> Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(input_path)?;
    let buff_reader = BufReader::new(file);
    Ok(buff_reader.lines().flatten())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
