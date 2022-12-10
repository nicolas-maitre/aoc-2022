use std::io::{stdout, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::{collections::HashSet, io::stdin};
use termion::event::{Event, Key};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

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
    const ROPE_SIZE: usize = 100;

    let mut rope_parts: [Position; ROPE_SIZE] = [(0, 0); ROPE_SIZE];

    loop {
        clear_scr();
        print_positions(&HashSet::from(rope_parts));

        let direction = get_arrows_direction();

        // move head
        match direction {
            Direction::Up => rope_parts[0].1 += 1,
            Direction::Left => rope_parts[0].0 -= 1,
            Direction::Down => rope_parts[0].1 -= 1,
            Direction::Right => rope_parts[0].0 += 1,
        }

        for part_index in 1..ROPE_SIZE {
            rope_parts[part_index].follow(rope_parts[part_index - 1]);
            // clear_scr();
            // print_positions(&HashSet::from(rope_parts));
            // sleep(Duration::from_millis(10));
        }
    }
}

fn print_positions(positions: &HashSet<Position>) {
    for y in (-25..25).rev() {
        for x in -50..50 {
            if positions.contains(&(x, y)) {
                print!("# ");
            } else {
                print!(". ");
            }
        }
        println!();
    }
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}
fn get_arrows_direction() -> Direction {
    loop {
        let stdin = stdin();
        let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
        stdout.flush().unwrap();
        for c in stdin.events() {
            return match c.unwrap() {
                Event::Key(Key::Left) => Direction::Left,
                Event::Key(Key::Right) => Direction::Right,
                Event::Key(Key::Up) => Direction::Up,
                Event::Key(Key::Down) => Direction::Down,
                Event::Key(Key::Ctrl('c')) => {
                    clear_scr();
                    stdout.flush().unwrap();
                    exit(0);
                    panic!();
                }
                _ => {
                    stdout.flush().unwrap();
                    continue;
                }
            };
        }
    }
}

fn clear_scr() {
    print!("\x1B[2J\x1B[1;1H");
    // println!();
}
