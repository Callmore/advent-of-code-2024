use regex::Regex;

const PUZZLE_INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("{}", process_puzzle(PUZZLE_INPUT));
}

fn process_puzzle(input: &str) -> i32 {
    let mut total = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for m in re.captures_iter(input) {
        total += dbg!(m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap());
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let example_input = include_str!("../../example_input.txt");
        let result = process_puzzle(example_input);
        assert_eq!(result, 161);
    }
}
