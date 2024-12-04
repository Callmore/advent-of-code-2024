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
            if chr == 'X' {
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
        let can_up = idx / puzzle_width >= 3;
        let can_down = idx / puzzle_width < puzzle_height - 3;
        let can_left = idx % puzzle_width >= 3;
        let can_right = idx % puzzle_width < puzzle_width - 3;

        if can_up {
            found += find_xmas(idx, up_offset, &puzzle) as i32;

            if can_left {
                found += find_xmas(idx, up_offset + left_offset, &puzzle) as i32;
            }
            if can_right {
                found += find_xmas(idx, up_offset + right_offset, &puzzle) as i32;
            }
        }
        if can_down {
            found += find_xmas(idx, down_offset, &puzzle) as i32;

            if can_left {
                found += find_xmas(idx, down_offset + left_offset, &puzzle) as i32;
            }
            if can_right {
                found += find_xmas(idx, down_offset + right_offset, &puzzle) as i32;
            }
        }
        if can_left {
            found += find_xmas(idx, left_offset, &puzzle) as i32;
        }
        if can_right {
            found += find_xmas(idx, right_offset, &puzzle) as i32;
        }
    }

    found
}

fn find_xmas(start_loc: usize, offset: isize, puzzle: &Vec<char>) -> bool {
    assert_eq!(puzzle[start_loc], 'X');

    puzzle[start_loc.checked_add_signed(offset).unwrap()] == 'M'
        && puzzle[start_loc.checked_add_signed(offset * 2).unwrap()] == 'A'
        && puzzle[start_loc.checked_add_signed(offset * 3).unwrap()] == 'S'
}

#[cfg(test)]
mod tests {
    use aoc::load_example_input;

    use super::*;

    #[test]
    fn example_input() -> Result<(), Box<dyn Error>> {
        let example_input = load_example_input()?;
        let result = process_puzzle(&example_input);
        assert_eq!(result, 18);
        Ok(())
    }
}
