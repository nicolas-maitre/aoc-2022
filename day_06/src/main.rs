use std::{collections::HashMap, fs::read_to_string};

fn main() {
    // let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let input = read_to_string("input.txt").unwrap();

    let pos = get_marker_pos(&input).unwrap();

    println!("Pos: {pos}");
}

fn get_marker_pos(input: &str) -> Option<usize> {
    'outer: for index in 0..input.len() {
        //part 1
        const MARKER_SIZE: usize = 4;
        //part 2
        // const MARKER_SIZE: usize = 14;

        let end_index = MARKER_SIZE + index;
        let slice = input
            .get(index..end_index)
            .expect("to not overrun the string");

        let mut count_by_char: HashMap<char, usize> = HashMap::new();
        for char in slice.chars() {
            let found_count = count_by_char.insert(char, 1).unwrap_or(0);
            if found_count > 0 {
                continue 'outer;
            }
        }
        // println!("slice: {}, index: {}..{}", slice, index, end_index);
        return Some(end_index);
    }
    None
}
