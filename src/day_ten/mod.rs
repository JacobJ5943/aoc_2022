use color_eyre::{Report, Result};

fn part_one<I>(input: I) -> Result<isize>
where
    I: Iterator<Item = String>,
{
    // I'm so sick of parsing strings
    let mut cycle_number = 0;
    let mut x_register = 1;
    let mut signal_sum = 0;
    for line in input {
        let mut split = line.trim().split(' ');
        match (split.next(), split.next()) {
            (Some("noop"), None) => {
                cycle_number += 1;
                match cycle_number {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        signal_sum += cycle_number * x_register;
                    }
                    _ => (),
                }
            }

            (Some("addx"), Some(add_x_value)) => {
                cycle_number += 1;
                match cycle_number {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        signal_sum += dbg!((dbg!(cycle_number)) * dbg!(x_register));
                    }
                    _ => (),
                }
                cycle_number += 1;
                match cycle_number {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        signal_sum += cycle_number * x_register;
                    }
                    _ => (),
                }
                x_register = addx(add_x_value.parse()?, x_register);
            }
            (_, _) => unimplemented!(),
        }
    }
    Ok(signal_sum)
}
fn addx(value: isize, x_register: isize) -> isize {
    x_register + value
}

fn part_two<I>(input: I) -> Result<Vec<String>>
where
    I: Iterator<Item = String>,
{
    // I'm so sick of parsing strings
    let mut cycle_number = 0;
    let mut x_register = 1;
    let mut pixels: Vec<char> = vec![];
    for line in input {
        let mut split = line.trim().split(' ');
        match (split.next(), split.next()) {
            (Some("noop"), None) => {
                cycle_number += 1;
                draw_pixel(&mut pixels, x_register, cycle_number);
            }

            (Some("addx"), Some(add_x_value)) => {
                cycle_number += 1;
                draw_pixel(&mut pixels, x_register, cycle_number);
                cycle_number += 1;
                draw_pixel(&mut pixels, x_register, cycle_number);
                x_register = addx(add_x_value.parse()?, x_register);
            }
            (_, _) => unimplemented!(),
        }
    }
    let mut pixels_iter = pixels.into_iter();
    let mut return_vec = Vec::new();

    while let Some(line) = (&mut pixels_iter)
        .take(40)
        .map(|c| c.to_string())
        .reduce(|acc, next| acc + &next)
    {
        return_vec.push(line);
    }

    Ok(return_vec)
}

fn draw_pixel(pixels: &mut Vec<char>, x_register: isize, cycle_number: isize) {
    let pixel_location = (cycle_number - 1) % 40; // -1 since cycle is indexed at 1 while pixels array is indexed at 0

    if pixel_location == x_register - 1
        || pixel_location == x_register
        || pixel_location == x_register + 1
    {
        pixels.push('#')
    } else {
        pixels.push('.')
    }

    //dbg!((pixel_number,x_register, pixel_location, pixels.last()));
}

#[cfg(test)]
mod tests {
    use crate::day_ten::part_two;

    use super::part_one;

    #[test]
    fn test_part_one_example() {
        let lines = vec![
            "addx 15".to_string(),
            "addx -11".to_string(),
            "addx 6".to_string(),
            "addx -3".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -8".to_string(),
            "addx 13".to_string(),
            "addx 4".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -35".to_string(),
            "addx 1".to_string(),
            "addx 24".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 16".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 21".to_string(),
            "addx -15".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -3".to_string(),
            "addx 9".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 8".to_string(),
            "addx 1".to_string(),
            "addx 5".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -36".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 6".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 7".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx 13".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx -33".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 8".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 17".to_string(),
            "addx -9".to_string(),
            "addx 1".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "addx 26".to_string(),
            "addx -30".to_string(),
            "addx 12".to_string(),
            "addx -1".to_string(),
            "addx 3".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -9".to_string(),
            "addx 18".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 9".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx -37".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "noop".to_string(),
            "addx 15".to_string(),
            "addx -21".to_string(),
            "addx 22".to_string(),
            "addx -6".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -10".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 20".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "addx 2".to_string(),
            "addx -6".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
        ];

        assert_eq!(part_one(lines.into_iter()).unwrap(), 13140);
    }

    #[test]
    fn test_part_one_my_input() {
        assert_eq!(
            part_one(crate::read_file_line_by_line("src/day_ten/input.txt").unwrap()).unwrap(),
            14780
        )
    }

    #[test]
    fn test_part_two_example() {
        let lines = vec![
            "addx 15".to_string(),
            "addx -11".to_string(),
            "addx 6".to_string(),
            "addx -3".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -8".to_string(),
            "addx 13".to_string(),
            "addx 4".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx 5".to_string(),
            "addx -1".to_string(),
            "addx -35".to_string(),
            "addx 1".to_string(),
            "addx 24".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 16".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 21".to_string(),
            "addx -15".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -3".to_string(),
            "addx 9".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 8".to_string(),
            "addx 1".to_string(),
            "addx 5".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -36".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 6".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 7".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx 13".to_string(),
            "addx 7".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "addx -33".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 8".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 17".to_string(),
            "addx -9".to_string(),
            "addx 1".to_string(),
            "addx 1".to_string(),
            "addx -3".to_string(),
            "addx 11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -13".to_string(),
            "addx -19".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "addx 26".to_string(),
            "addx -30".to_string(),
            "addx 12".to_string(),
            "addx -1".to_string(),
            "addx 3".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -9".to_string(),
            "addx 18".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 9".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx -1".to_string(),
            "addx 2".to_string(),
            "addx -37".to_string(),
            "addx 1".to_string(),
            "addx 3".to_string(),
            "noop".to_string(),
            "addx 15".to_string(),
            "addx -21".to_string(),
            "addx 22".to_string(),
            "addx -6".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx 2".to_string(),
            "addx 1".to_string(),
            "noop".to_string(),
            "addx -10".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "addx 20".to_string(),
            "addx 1".to_string(),
            "addx 2".to_string(),
            "addx 2".to_string(),
            "addx -6".to_string(),
            "addx -11".to_string(),
            "noop".to_string(),
            "noop".to_string(),
            "noop".to_string(),
        ];

        assert_eq!(
            part_two(lines.into_iter()).unwrap(),
            vec![
                "##..##..##..##..##..##..##..##..##..##..".to_string(),
                "###...###...###...###...###...###...###.".to_string(),
                "####....####....####....####....####....".to_string(),
                "#####.....#####.....#####.....#####.....".to_string(),
                "######......######......######......####".to_string(),
                "#######.......#######.......#######.....".to_string()
            ]
        );
    }

    #[test]
    fn test_part_two_my_input() {
        let expetced = vec![
            "####.#....###..#....####..##..####.#....".to_string(),
            "#....#....#..#.#.......#.#..#....#.#....".to_string(),
            "###..#....#..#.#......#..#......#..#....".to_string(),
            "#....#....###..#.....#...#.##..#...#....".to_string(),
            "#....#....#....#....#....#..#.#....#....".to_string(),
            "####.####.#....####.####..###.####.####.".to_string(),
        ];
        assert_eq!(
            part_two(crate::read_file_line_by_line("src/day_ten/input.txt").unwrap()).unwrap(),
            expetced
        )
    }
}
