use std::{collections::HashMap, fs::read_to_string};

fn main() {
    //     let str = "vJrwpWtwJgWrhcsFMMfFFhFp;
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw
    // ";
    let str = read_to_string("input.txt").unwrap();

    //a:97 z:122 A:65 Z:90

    let rucksacks = str.split("\n").map_while(|ruck_str| {
        let strlen = ruck_str.len();
        if strlen == 0 {
            return None;
        }

        let compartments = ruck_str.split_at(strlen / 2);

        let mut items_by_count: HashMap<char, u32> = HashMap::new();
        compartments.0.chars().for_each(|char| {
            items_by_count.insert(char, 1);
        });
        compartments.1.chars().for_each(|char| {
            let count = items_by_count.get(&char).unwrap_or(&0).to_owned();
            items_by_count.insert(char, if count >= 1 { 2 } else { count });
        });

        let found_many = items_by_count
            .iter()
            .filter(|(_, count)| (count.to_owned().to_owned() >= 2))
            .map(|(char, _)| char);

        let totals = found_many.map(|char| get_char_num(char.to_owned()));

        let total = totals.reduce(|a, b| a + b).unwrap_or(0);

        return Some(total);
    });

    let total = rucksacks.reduce(|a, b| a + b).unwrap_or(0);

    println!("total {}", total);
}

fn get_char_num(char: char) -> u32 {
    const L_A_ASCII: u32 = 'a' as u32;
    const L_Z_ASCII: u32 = 'z' as u32;
    const U_A_ASCII: u32 = 'A' as u32;
    const U_Z_ASCII: u32 = 'Z' as u32;
    match char as u32 {
        (L_A_ASCII..=L_Z_ASCII) => (char as u32) - L_A_ASCII + 1,
        (U_A_ASCII..=U_Z_ASCII) => (char as u32) - U_A_ASCII + 27,
        _ => panic!("invalid char '{}'", char),
    }
}
