use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut sorted = input
        .split("\n\n")
        .map(|strpart| {
            strpart
                .split("\n")
                .fold(0, |acc, curr| curr.parse::<i32>().unwrap_or(0) + acc)
        })
        .collect::<Vec<i32>>();

    sorted.sort();

    println!(
        "{}",
        sorted
            .into_iter()
            .rev()
            .take(3)
            .reduce(|a, b| a + b)
            .unwrap()
    )
}
