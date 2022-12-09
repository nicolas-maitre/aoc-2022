use std::{collections::HashSet, fs::read_to_string};

type Position = (i32, i32);

trait Chain {
    fn follow(&mut self, folowee: Self);
}
impl Chain for Position {
    fn follow(&mut self, folowee: Self) {
        //move tail x
        if (folowee.0 - self.0).abs() > 1 {
            if folowee.1 != self.1 {
                self.1 = folowee.1;
            }
            if folowee.0 > self.0 {
                self.0 += 1;
            } else {
                self.0 -= 1;
            }
        }
        //move tail y
        if (folowee.1 - self.1).abs() > 1 {
            if folowee.0 != self.0 {
                self.0 = folowee.0;
            }
            if folowee.1 > self.1 {
                self.1 += 1;
            } else {
                self.1 -= 1;
            }
        }
    }
}

fn main() {
    //     let input = "R 4
    // U 4
    // L 3
    // D 1
    // R 4
    // D 1
    // L 5
    // R 2"; //= 13

    let input = read_to_string("../input.txt").expect("input file");

    let mut head_pos: Position = (0, 0);
    let mut tail_pos: Position = head_pos;
    let mut visited: HashSet<Position> = HashSet::new();

    for instruction in input.lines() {
        let (direction, steps) = instruction.split_once(" ").expect("valid instruction");
        //sub steps
        for _ in 0..steps.parse().expect("valid step count") {
            //not optimised omg put this outside the loop omg wtf
            //move head
            match direction {
                "U" => head_pos.1 += 1,
                "L" => head_pos.0 -= 1,
                "D" => head_pos.1 -= 1,
                "R" => head_pos.0 += 1,
                _ => panic!("invalid direction {}", direction),
            }

            tail_pos.follow(head_pos);

            // DEBUG
            // print_positions(&HashSet::from([head_pos, tail_pos]));
            // println!();

            visited.insert(tail_pos);
        }
    }

    // print_positions(&visited);
    println!("visited count: {}", visited.len());
}

fn print_positions(positions: &HashSet<Position>) {
    for y in (0..5).rev() {
        for x in 0..6 {
            if positions.contains(&(x, y)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}
