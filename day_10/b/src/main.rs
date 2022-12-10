use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input.txt").expect("valid input file");
    let mut total_cycles = 0;
    let mut x_register = 1;
    let mut add_cycles = |count, curr_x: i32| {
        for _ in 0..count {
            let row = total_cycles % 40;
            total_cycles += 1;
            let diff = (curr_x - row).abs();
            if diff == 0 {
                print!("#");
            } else if diff == 1 {
                print!("=");
            } else if diff == 2 {
                print!(".");
            } else {
                print!(" ");
            }
            let row = total_cycles % 40;
            if row == 0 {
                println!();
            }
        }
    };

    for command_line in input.lines() {
        let command = &command_line[0..4];
        match command {
            "addx" => {
                add_cycles(2, x_register);
                let value: i32 = command_line
                    .get(5..)
                    .expect("value argument to addx")
                    .parse()
                    .expect("valid number");

                x_register += value;
            }
            "noop" => add_cycles(1, x_register),
            _ => panic!("invalid command {}", command),
        }
    }
    println!();
    println!("x register: {}", x_register);
    println!("cycles: {}", total_cycles);
}
