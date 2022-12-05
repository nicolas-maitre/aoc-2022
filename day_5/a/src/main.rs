use std::fs::read_to_string;

fn main() {
    //     let input = "    [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3

    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    // move 1 from 1 to 2
    // ";

    let input = read_to_string("../input.txt").expect("valid file");

    let (boxes_str, commands_str) = input.split_once("\n\n").expect("two inputs");

    //___BOXES___
    //parse boxes: group chars
    let box_lines = boxes_str.lines().filter_map(|box_line| {
        const CHUNK_LEN: usize = "[_] ".len();
        let chars: Vec<char> = box_line.chars().collect();
        let box_places = chars.chunks(CHUNK_LEN).map(|chars| {
            if chars.get(0).unwrap_or(&' ') == &'[' {
                Some(chars[1])
            } else {
                None
            }
        });

        //trim useless lines
        if box_places.clone().all(|bp| bp.is_none()) {
            None
        } else {
            Some(box_places.collect::<Vec<Option<char>>>())
        }
    });

    //stacks bottom up
    let mut box_stacks: Vec<Vec<char>> = vec![];
    box_lines.rev().for_each(|line| {
        line.iter().enumerate().for_each(|(index, box_place)| {
            if let Some(box_char) = box_place.to_owned() {
                if let Some(stack) = box_stacks.get_mut(index) {
                    stack.push(box_char);
                } else {
                    box_stacks.push(vec![box_char]);
                }
            }
        });
    });
    // println!("box_stacks {:?}", box_stacks);

    //___COMMANDS___
    commands_str.lines().for_each(|command_line| {
        //parse command
        let (_, rest) = command_line.split_once("move ").expect("valid command (1)");
        let (move_str, rest) = rest.split_once(" from ").expect("valid command (2)");
        let (from_str, to_str) = rest.split_once(" to ").expect("valid command (3)");
        let move_count: usize = move_str.parse().expect("valid command move count");
        let source_stack_index: usize = from_str.parse().expect("valid command source stack index");
        let destination_stack_index: usize =
            to_str.parse().expect("valid command dest. stack index");

        let (source_index, destination_index) =
            (source_stack_index - 1, destination_stack_index - 1);

        let mut source_stack = box_stacks[source_index].clone();
        let removed = (0..move_count).map(|_| source_stack.pop().unwrap());
        let mut destination_stack = box_stacks[destination_index].clone();
        destination_stack.extend(removed);

        box_stacks[source_index] = source_stack.to_vec();
        box_stacks[destination_index] = destination_stack;

        // DEBUG
        // println!("____");
        // println!("command: {}", command_line);
        // println!("stacks: {:?}", box_stacks);
    });

    //___RESULT___
    let top_boxes = box_stacks.iter().filter_map(|stack| stack.last());
    let top_boxes_str = top_boxes
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("result: {:?}", top_boxes_str);
}
