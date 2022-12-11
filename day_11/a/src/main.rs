use std::fs::read_to_string;

trait NumVecFromStr {
    fn from_str(str: &str) -> Self;
}
impl NumVecFromStr for Vec<i32> {
    fn from_str(str: &str) -> Self {
        str.split(", ")
            .map(|n| n.parse().expect("valid start item number"))
            .collect()
    }
}

#[derive(Debug)]
enum Operation {
    Add(i32),
    Multiply(i32),
}
impl Operation {
    fn from_str(str: &str) -> Self {
        let (operator, num) = str.split_once(" ").expect("valid operation string");
        println!("{str}");
        let number: i32 = num.parse().expect("valid operation number");
        match operator {
            "+" => Self::Add(number),
            "*" => Self::Multiply(number),
            _ => panic!("invalid operator: {}", operator),
        }
    }
}

#[derive(Debug)]
enum Test {
    DivisibleBy(i32),
}
#[derive(Debug)]
struct Monkey {
    starting_items: Vec<i32>,
    operation: Operation,
    test: Test,
    if_true_monkey: i32,
    if_false_monkey: i32,
}
fn main() {
    let input = read_to_string("../small_input.txt").expect("valid input file");
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey_str| {
            let (_, rest) = monkey_str
                .split_once("Starting items: ")
                .expect("starting items line");
            let (starting_items_str, rest) = rest
                .split_once("\n  Operation: new = old ")
                .expect("operation line");
            let (operation_str, rest) = rest
                .split_once("\n  Test: divisible by ")
                .expect("test line");
            let (divisible_by_str, rest) = rest
                .split_once("\n    If true: throw to monkey ")
                .expect("if true line");
            let (if_true_monkey_str, if_false_monkey_str) = rest
                .split_once("\n    If false: throw to monkey ")
                .expect("if false line");

            return Monkey {
                starting_items: Vec::from_str(starting_items_str),
                operation: Operation::from_str(operation_str),
                test: Test::DivisibleBy(divisible_by_str.parse().expect("valid divisor")),
                if_true_monkey: if_true_monkey_str.parse().expect("valid true monkey"),
                if_false_monkey: if_false_monkey_str.parse().expect("valid false monkey"),
            };
        })
        .collect();
    println!("{:#?}", monkeys);
}
