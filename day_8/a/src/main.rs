fn main() {
    let input = "30373
25512
65332
33549
35390
"; //=21

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("a valid char"))
                .collect()
        })
        .collect();

    let grid_height = grid.len();
    let grid_width = grid.get(0).expect("at leat one line").len();

    //h check
    for x in 0..grid_width {}
    //v check
    for y in 0..grid_height {}
}
