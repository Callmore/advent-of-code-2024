use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../../input.txt");

fn main() {
    let mut numbers: Vec<i32> = vec![];
    let mut frequency: HashMap<i32, i32> = HashMap::new();

    for line in PUZZLE_INPUT.lines() {
        let mut line_iter = line.split_whitespace();
        numbers.push(line_iter.next().unwrap().parse().unwrap());
        let n: i32 = line_iter.next().unwrap().parse().unwrap();
        *frequency.entry(n).or_insert(0) += 1;
    }

    let mut similarity = 0;

    for number in numbers {
        similarity += frequency.get(&number).or(Some(&0)).unwrap() * number;
    }

    println!("{}", similarity);
}
