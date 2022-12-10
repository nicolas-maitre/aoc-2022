use std::fs::read_to_string;

fn main() {
    let input = read_to_string("../input.txt").expect("valid input file");
    let mut total_cycles = 0;
    let mut x_register = 1;
    let mut total_signal_strength = 0;
    let mut add_cycles = |count, curr_x| {
        for _ in 0..count {
            total_cycles += 1;
            if total_cycles % 40 == 20 {
                total_signal_strength += total_cycles * curr_x;
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
    println!("x register: {}", x_register);
    println!("cycles: {}", total_cycles);
    println!("signal_strength: {}", total_signal_strength);
}
