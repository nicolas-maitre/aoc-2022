use std::{collections::HashSet, fs::read_to_string};

type Position = (i32, i32);

trait Chain {
    fn follow(&mut self, folowee: Self);
}
impl Chain for Position {
    fn follow(&mut self, folowee: Self) {
        let has_diff_x = (folowee.0 - self.0).abs() > 1;
        let has_diff_y = (folowee.1 - self.1).abs() > 1;

        //move tail x
        if has_diff_x {
            if folowee.0 > self.0 {
                self.0 += 1;
            } else {
                self.0 -= 1;
            }

            if !has_diff_y && folowee.1 != self.1 {
                self.1 = folowee.1;
            }
        }

        //move tail y
        if has_diff_y {
            if folowee.1 > self.1 {
                self.1 += 1;
            } else {
                self.1 -= 1;
            }

            if !has_diff_x && folowee.0 != self.0 {
                self.0 = folowee.0;
            }
        }
    }
}

fn main() {
    //     let input = "R 5
    // U 8
    // L 8
    // D 3
    // R 17
    // D 10
    // L 25
    // U 20"; //= 13

    let input = read_to_string("../input.txt").expect("input file");

    const ROPE_SIZE: usize = 10;

    let mut rope_parts: [Position; ROPE_SIZE] = [(0, 0); ROPE_SIZE];

    let mut visited: HashSet<Position> = HashSet::new();

    for instruction in input.lines() {
        let (direction, steps) = instruction.split_once(" ").expect("valid instruction");
        //sub steps
        for _ in 0..steps.parse().expect("valid step count") {
            //not optimised omg put this outside the loop omg wtf
            //move head
            match direction {
                "U" => rope_parts[0].1 += 1,
                "L" => rope_parts[0].0 -= 1,
                "D" => rope_parts[0].1 -= 1,
                "R" => rope_parts[0].0 += 1,
                _ => panic!("invalid direction {}", direction),
            }

            for part_index in 1..ROPE_SIZE {
                rope_parts[part_index].follow(rope_parts[part_index - 1])
            }

            visited.insert(rope_parts.last().unwrap().to_owned());
        }
        // DEBUG
        // print_positions(&HashSet::from(rope_parts));
        // println!();
    }

    // print_positions(&visited);
    println!("visited count: {}", visited.len());
}

fn print_positions(positions: &HashSet<Position>) {
    for y in (-5..15).rev() {
        for x in -15..15 {
            if positions.contains(&(x, y)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
