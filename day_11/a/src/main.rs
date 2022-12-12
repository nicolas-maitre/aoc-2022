use std::fs::read_to_string;

trait NumVecFromStr {
    fn from_str(str: &str) -> Self;
}
impl NumVecFromStr for Vec<i64> {
    fn from_str(str: &str) -> Self {
        str.split(", ")
            .map(|n| n.parse().expect("valid start item number"))
            .collect()
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Double,
    Square,
}
impl Operation {
    fn execute(&self, old: &i64) -> i64 {
        match self {
            Self::Add(num) => num.checked_add(*old).expect("no add overflow"),
            Self::Multiply(num) => num.checked_mul(*old).expect("no mul overflow"),
            Self::Double => old.checked_mul(2).expect("no double overflow"),
            Self::Square => old.checked_mul(*old).expect("no square overflow"),
        }
    }
    fn from_str(str: &str) -> Self {
        let (operator, num) = str.split_once(" ").expect("valid operation string");
        if num == "old" {
            return match operator {
                "+" => Self::Double,
                "*" => Self::Square,
                _ => panic!("invalid operator: {}", operator),
            };
        }
        let number: i64 = num.parse().expect("valid operation number");
        match operator {
            "+" => Self::Add(number),
            "*" => Self::Multiply(number),
            _ => panic!("invalid operator: {}", operator),
        }
    }
}

#[derive(Debug, Clone)]
enum Test {
    DivisibleBy(i64),
}
impl Test {
    fn execute(&self, value: &i64) -> bool {
        match self {
            Self::DivisibleBy(num) => value % num == 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: Test,
    if_true_monkey: usize,
    if_false_monkey: usize,
    items_inspected_count: usize,
}
impl Monkey {
    fn from_str(str: &str) -> Self {
        let (_, rest) = str
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
        let (if_true_monkey_str, rest) = rest
            .split_once("\n    If false: throw to monkey ")
            .expect("if false line");
        let if_false_monkey_str = rest.trim();

        return Self {
            items: Vec::from_str(starting_items_str),
            operation: Operation::from_str(operation_str),
            test: Test::DivisibleBy(divisible_by_str.parse().expect("valid divisor")),
            if_true_monkey: if_true_monkey_str.parse().expect("valid true monkey"),
            if_false_monkey: if_false_monkey_str.parse().expect("valid false monkey"),
            items_inspected_count: 0,
        };
    }
}

fn main() {
    let input = read_to_string("../input.txt").expect("valid input file");
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|str| Monkey::from_str(str))
        .collect();

    for round_index in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys[monkey_index].clone();
            let mut updated_items: Vec<Option<i64>> =
                monkey.items.iter().map(|i| Some(i.to_owned())).collect();
            monkeys[monkey_index].items_inspected_count =
                monkey.items_inspected_count + monkey.items.len();
            for (item_index, old_item) in monkey.items.iter().enumerate() {
                let new_item = monkey.operation.execute(old_item) / 3;
                let result = monkey.test.execute(&new_item);
                let target_monkey_index = if result {
                    monkey.if_true_monkey
                } else {
                    monkey.if_false_monkey
                };
                updated_items[item_index] = None;
                if target_monkey_index == monkey_index {
                    updated_items.push(Some(new_item));
                } else {
                    monkeys[target_monkey_index].items.push(new_item);
                }
            }
            monkeys[monkey_index].items =
                updated_items.iter().filter_map(|i| i.to_owned()).collect();
        }

        //print monkeys
        println!("Round {}", round_index + 1);
        for (monkey_index, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", monkey_index, monkey.items);
        }
        println!();
    }
    //print monkeys
    for (monkey_index, monkey) in monkeys.iter().enumerate() {
        println!(
            "Monkey {} inspected {} items",
            monkey_index, monkey.items_inspected_count
        );
    }
    println!();
    let mut best_activities = monkeys
        .iter()
        .map(|monkey| monkey.items_inspected_count)
        .collect::<Vec<usize>>();
    best_activities.sort();
    let two_bests = best_activities.iter().rev().take(2).map(|i| i.to_owned());

    println!(
        "Monkey business: {}",
        two_bests.reduce(|o, n| o * n).expect("monkey business")
    );
}
