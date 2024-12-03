use regex::Regex;

const PUZZLE_INPUT: &str = include_str!("../../input.txt");

fn main() {
    println!("{}", process_puzzle(PUZZLE_INPUT));
}

fn process_puzzle(input: &str) -> i32 {
    let mut total = 0;
    let mut input_position = 0;

    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut is_enabled = true;

    loop {
        if is_enabled {
            let next_mul = mul_re.captures_at(input, input_position);
            let mul_loc = match next_mul {
                Some(ref c) => c.get(0).unwrap().start(),
                None => usize::MAX,
            };

            let next_dont = dont_re.find_at(input, input_position);
            let dont_loc = match next_dont {
                Some(m) => m.start(),
                None => usize::MAX,
            };

            // If they're equal, then no match, or SHIT IS BROKEN
            if mul_loc == dont_loc {
                assert_eq!(mul_loc, usize::MAX);
                break;
            }

            if mul_loc < dont_loc {
                let next_mul = next_mul.unwrap();
                total += next_mul[1].parse::<i32>().unwrap() * next_mul[2].parse::<i32>().unwrap();
                input_position = next_mul.get(0).unwrap().end();
            } else if dont_loc < mul_loc {
                is_enabled = false;
                input_position = next_dont.unwrap().end();
            }
        } else {
            let next_do = do_re.find_at(input, input_position);
            if next_do.is_none() {
                break;
            }
            input_position = next_do.unwrap().end();
            is_enabled = true;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let example_input = include_str!("../../example_b_input.txt");
        let result = process_puzzle(example_input);
        assert_eq!(result, 48);
    }
}
