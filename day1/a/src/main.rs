const PUZZLE_INPUT: &str = include_str!("../../input.txt");

fn main() {
    let mut lists: [Vec<i32>;2] = [vec![], vec![]];

    for line in PUZZLE_INPUT.lines() {
        let mut line_iter = line.split_whitespace();
        for entry in &mut lists {
            entry.push(line_iter.next().unwrap().parse().unwrap());
        }
    }

    for entry in &mut lists {
        entry.sort();
    }

    let [list_0, list_1] = lists;
    let total_sum = list_0.iter().zip(list_1).fold(0, |acc, (x, y)| acc + (x - y).abs());

    println!("{}", total_sum)
}
