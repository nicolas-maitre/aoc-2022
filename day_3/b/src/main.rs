use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let str = read_to_string("input.txt").unwrap();

    const GROUP_BY_COUNT: usize = 3;

    let lines = str.lines().collect::<Vec<&str>>();
    let rucksacks = lines.chunks(GROUP_BY_COUNT).map(|chunk| {
        let mut items_by_count: HashMap<char, usize> = HashMap::new();
        chunk.iter().enumerate().for_each(|(index, elf_sack)| {
            elf_sack.chars().for_each(|char| {
                let count = items_by_count.get(&char).unwrap_or(&0).to_owned();
                if count == index {
                    items_by_count.insert(char, count + 1);
                };
            });
        });

        let found_many = items_by_count
            .iter()
            .filter(|(_, count)| (count.to_owned().to_owned() >= GROUP_BY_COUNT))
            .map(|(char, _)| char);

        let totals = found_many.map(|char| get_char_num(char.to_owned()));

        totals.reduce(|a, b| a + b).unwrap_or(0)
    });

    let total = rucksacks.reduce(|a, b| a + b).unwrap_or(0);

    println!("total {}", total);
}

fn get_char_num(char: char) -> usize {
    const L_A_ASCII: usize = 'a' as usize;
    const L_Z_ASCII: usize = 'z' as usize;
    const U_A_ASCII: usize = 'A' as usize;
    const U_Z_ASCII: usize = 'Z' as usize;
    match char as usize {
        (L_A_ASCII..=L_Z_ASCII) => (char as usize) - L_A_ASCII + 1,
        (U_A_ASCII..=U_Z_ASCII) => (char as usize) - U_A_ASCII + 27,
        _ => panic!("invalid char '{}'", char),
    }
}
