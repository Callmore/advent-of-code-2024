const PUZZLE_INPUT: &str = include_str!("../../input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Undetermined,
    Increasing,
    Decreasing,
}

fn main() {
    let mut total_safe = 0;

    'scan_line: for line in PUZZLE_INPUT.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut direction = Direction::Undetermined;
        let mut can_dampened = true;
        let mut previous_number = numbers[0];
        for number in numbers[1..].into_iter() {
            let diff = *number - previous_number;

            if diff.abs() <= 3
                && (direction == Direction::Undetermined
                    || (direction == Direction::Increasing && diff > 0)
                    || (direction == Direction::Decreasing && diff < 0))
            {
                if direction == Direction::Undetermined {
                    direction = if diff > 0 {
                        Direction::Increasing
                    } else {
                        Direction::Decreasing
                    }
                }

                previous_number = *number;
            } else if can_dampened {
                can_dampened = false;
            } else {
                continue 'scan_line;
            }
        }
        total_safe += 1;
    }

    println!("{}", total_safe);
}
