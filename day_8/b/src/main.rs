use std::fs::read_to_string;

fn main() {
    //     let input = "30373
    // 25512
    // 65332
    // 33549
    // 35390
    // "; //=8

    let input = read_to_string("../input.txt").expect("valid input file");

    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("a valid char") as i32)
                .collect()
        })
        .collect();

    let grid_height = grid.len();
    let grid_width = grid.get(0).expect("at leat one line").len();

    let visibility_scores: Vec<Vec<i32>> = grid
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, tree_height)| {
                    //to right
                    let mut score_tr = 0;
                    'tr: for other_x in (x + 1)..grid_width {
                        let other_tree_height = grid[y][other_x];
                        score_tr += 1;
                        if &other_tree_height >= tree_height {
                            break 'tr;
                        }
                    }
                    let score = score_tr;

                    //to left
                    let mut score_tl = 0;
                    'tl: for other_x in (0..x).rev() {
                        let other_tree_height = grid[y][other_x];
                        score_tl += 1;
                        if &other_tree_height >= tree_height {
                            break 'tl;
                        }
                    }
                    let score = score * score_tl;

                    // to down
                    let mut score_td = 0;
                    'td: for other_y in (y + 1)..grid_height {
                        let other_tree_height = grid[other_y][x];
                        score_td += 1;
                        if &other_tree_height >= tree_height {
                            break 'td;
                        }
                    }
                    let score = score * score_td;

                    // to up
                    let mut score_tu = 0;
                    'tu: for other_y in (0..y).rev() {
                        let other_tree_height = grid[other_y][x];
                        score_tu += 1;
                        if &other_tree_height >= tree_height {
                            break 'tu;
                        }
                    }
                    score * score_tu
                })
                .collect()
        })
        .collect();

    // pretty_print_vec_nums(&grid);
    // println!("====");
    // pretty_print_vec_nums(&visibility_scores);

    let best_score = visibility_scores.iter().fold(0, |curr_best, line| {
        curr_best.max(line.iter().max().unwrap().to_owned())
    });

    println!("best score: {}", best_score);
}

fn pretty_print_vec_nums(vec_nums: &Vec<Vec<i32>>) {
    for line in vec_nums {
        for num in line {
            print!("{:2}", num);
        }
        println!("");
    }
}
