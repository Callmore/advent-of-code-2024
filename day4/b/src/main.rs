use std::error::Error;

use aoc::load_input;

fn main() -> Result<(), Box<dyn Error>> {
    let input = load_input()?;

    println!("{}", process_puzzle(&input));

    Ok(())
}

fn process_puzzle(input: &str) -> i32 {
    let mut found = 0;
    // puzzle is the puzzle input stored in rows
    let mut puzzle: Vec<char> = vec![];
    // puzzle_width is the width of the puzzle
    let mut puzzle_width = 0usize;
    // x_map is a list of indexes where an x was found
    let mut x_map: Vec<usize> = vec![];

    for line in input.lines() {
        for chr in line.chars() {
            if chr == 'A' {
                x_map.push(puzzle.len());
            }
            puzzle.push(chr);
        }
        if puzzle_width == 0 {
            puzzle_width = puzzle.len();
        }
    }

    let puzzle_height = puzzle.len() / puzzle_width;

    let up_offset = -isize::try_from(puzzle_width).unwrap();
    let down_offset = isize::try_from(puzzle_width).unwrap();
    let left_offset = -1isize;
    let right_offset = 1isize;

    for idx in x_map {
        if idx / puzzle_width <= 0 || idx / puzzle_width >= puzzle_height - 1 {
            continue;
        }
        if idx % puzzle_width <= 0 || idx % puzzle_width >= puzzle_width - 1 {
            continue;
        }

        // M M
        //  A
        // S S
        found += find_x_mas(
            idx,
            [
                up_offset + left_offset,
                up_offset + right_offset,
                down_offset + left_offset,
                down_offset + right_offset,
            ],
            &puzzle,
        ) as i32;

        // S S
        //  A
        // M M
        found += find_x_mas(
            idx,
            [
                down_offset + left_offset,
                down_offset + right_offset,
                up_offset + left_offset,
                up_offset + right_offset,
            ],
            &puzzle,
        ) as i32;

        // M S
        //  A
        // M S
        found += find_x_mas(
            idx,
            [
                up_offset + left_offset,
                down_offset + left_offset,
                up_offset + right_offset,
                down_offset + right_offset,
            ],
            &puzzle,
        ) as i32;

        // S M
        //  A
        // S M
        found += find_x_mas(
            idx,
            [
                up_offset + right_offset,
                down_offset + right_offset,
                up_offset + left_offset,
                down_offset + left_offset,
            ],
            &puzzle,
        ) as i32;
    }

    found
}

///offsets: M M S S
fn find_x_mas(start_loc: usize, offsets: [isize; 4], puzzle: &Vec<char>) -> bool {
    assert_eq!(puzzle[start_loc], 'A');

    puzzle[start_loc.checked_add_signed(offsets[0]).unwrap()] == 'M'
        && puzzle[start_loc.checked_add_signed(offsets[1]).unwrap()] == 'M'
        && puzzle[start_loc.checked_add_signed(offsets[2]).unwrap()] == 'S'
        && puzzle[start_loc.checked_add_signed(offsets[3]).unwrap()] == 'S'
}

#[cfg(test)]
mod tests {
    use aoc::load_example_input;

    use super::*;

    #[test]
    fn example_input() -> Result<(), Box<dyn Error>> {
        let example_input = load_example_input()?;
        let result = process_puzzle(&example_input);
        assert_eq!(result, 9);
        Ok(())
    }
}
