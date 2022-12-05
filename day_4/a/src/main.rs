use std::fs::read_to_string;

fn main() {
    //     let input = "2-4,6-8
    // 2-3,4-5
    // 5-7,7-9
    // 2-8,3-7
    // 6-6,4-6
    // 2-6,4-8
    // ";

    let input = read_to_string("../input.txt").expect("file error");

    let pairs_contain_count = input.lines().map(|pair_str| {
        let assignment_strings = pair_str.split(",");
        let ranges = assignment_strings.map(|assignment_str| {
            let str_parts: Vec<&str> = assignment_str.split("-").collect();
            let start: usize = str_parts[0].parse().expect("invalid start num");
            let end: usize = str_parts[1].parse().expect("invalid start num");
            start..=end
        });

        //yes, iterating here is overkill since there are only pairs
        let ranges_iter = ranges.enumerate();
        let overlap_count = ranges_iter.clone().fold(0, |count, (index, range)| {
            let sub_overlap_count =
                ranges_iter
                    .clone()
                    .fold(0, |sub_count, (sub_index, sub_range)| {
                        if index != sub_index
                            && range.contains(sub_range.start())
                            && range.contains(sub_range.end())
                        {
                            sub_count + 1
                        } else {
                            sub_count
                        }
                    });
            count + sub_overlap_count
        });
        overlap_count
    });

    let fully_contained_pair_count =
        pairs_contain_count.fold(
            0,
            |count, pair_count| if pair_count > 0 { count + 1 } else { count },
        );

    println!("count: {}", fully_contained_pair_count);
}
