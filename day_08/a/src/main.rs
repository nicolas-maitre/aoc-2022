use std::fs::read_to_string;

fn main() {
    //     let input = "30373
    // 25512
    // 65332
    // 33549
    // 35390
    // "; //=21

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

    let mut visible_trees = vec![vec![false; grid_width]; grid_height];

    //h check
    for y in 0..grid_height {
        //ltr
        let mut max_ltr = -1;
        for x in 0..grid_width {
            if grid[y][x] > max_ltr {
                max_ltr = grid[y][x];
                visible_trees[y][x] = true;
            }
        }
        //rtl
        let mut max_rtl = -1;
        for x in (0..grid_width).rev() {
            if grid[y][x] > max_rtl {
                visible_trees[y][x] = true;
                if grid[y][x] == max_ltr {
                    break;
                }
                max_rtl = grid[y][x];
            }
        }
    }
    //v check
    for x in 0..grid_width {
        //utd
        let mut max_utd = -1;
        for y in 0..grid_height {
            if grid[y][x] > max_utd {
                max_utd = grid[y][x];
                visible_trees[y][x] = true;
            }
        }
        //dtu
        let mut max_dtu = -1;
        for y in (0..grid_height).rev() {
            if grid[y][x] > max_dtu {
                visible_trees[y][x] = true;
                if grid[y][x] == max_utd {
                    break;
                }
                max_dtu = grid[y][x];
            }
        }
    }

    let total_visible = visible_trees.iter().fold(0, |count, vec| {
        count
            + vec.iter().fold(0, |count, is_visible| {
                if is_visible.to_owned() {
                    count + 1
                } else {
                    count
                }
            })
    });

    pretty_print_visible(&visible_trees);

    println!("total visible: {}", total_visible);
}

fn pretty_print_visible(visible_trees: &Vec<Vec<bool>>) {
    for line in visible_trees {
        for visible in line {
            if visible.to_owned() {
                print!("V")
            } else {
                print!("_")
            }
        }
        println!("");
    }
}
