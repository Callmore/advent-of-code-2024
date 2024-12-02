const PUZZLE_INPUT: &str = include_str!("../../input.txt");

fn main() {
    let mut total_safe = 0;
    for line in PUZZLE_INPUT.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        if (numbers.iter().all(|x| *x > 0) || numbers.iter().all(|x| *x < 0))
            && numbers.iter().all(|x| (*x).abs() <= 3)
        {
            total_safe += 1;
        }
    }

    println!("{}", total_safe);
}
