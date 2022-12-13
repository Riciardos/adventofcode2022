mod day11 {}

use super::util::read_lines;

pub fn main() {
    let mut monkeys = setup_monkeys();

    for i in 0..10000 {
        for j in 0..monkeys.len() {
            let items = monkeys[j].items.clone();
            for k in 0..items.len() {
                let result = (monkeys[j].operation)(&items[k]);
                let (new_monkey_idx, divide_result) = (monkeys[j].test)(result);
                monkeys[new_monkey_idx as usize].items.push(divide_result);
                monkeys[j].inspections += 1;
            }
            monkeys[j].items = vec![];
        }
    }

    monkeys.sort_by(|x, y| y.inspections.cmp(&x.inspections));

    println!(
        "Day 11 result {}",
        monkeys[0].inspections * monkeys[1].inspections
    );
}

#[derive(Clone)]
struct Monkey {
    idx: u32,
    items: Vec<i64>,
    operation: fn(&i64) -> i64,
    test: fn(i64) -> (u32, i64),
    inspections: u64,
}

fn setup_test_monkeys() -> Vec<Monkey> {
    let monkey_0 = Monkey {
        idx: 0,
        items: vec![79, 98],
        operation: |x| -> i64 { x * 19 },
        test: |x| divide_and_pick_monkey(x, 23, 2, 3),
        inspections: 0,
    };

    let monkey_1 = Monkey {
        idx: 1,
        items: vec![54, 65, 75, 74],
        operation: |x| -> i64 { x + 6 },
        test: |x| divide_and_pick_monkey(x, 19, 2, 0),
        inspections: 0,
    };

    let monkey_2 = Monkey {
        idx: 2,
        items: vec![79, 60, 97],
        operation: |x| -> i64 { x * x },
        test: |x| divide_and_pick_monkey(x, 13, 1, 3),
        inspections: 0,
    };

    let monkey_3 = Monkey {
        idx: 3,
        items: vec![74],
        operation: |x| -> i64 { x + 3 },
        test: |x| divide_and_pick_monkey(x, 17, 0, 1),
        inspections: 0,
    };

    vec![monkey_0, monkey_1, monkey_2, monkey_3]
}

fn setup_monkeys() -> Vec<Monkey> {
    let monkey_0 = Monkey {
        idx: 0,
        items: vec![77, 69, 76, 77, 50, 58],
        operation: |x| -> i64 { x * 11 },
        test: |x| divide_and_pick_monkey(x, 5, 1, 5),
        inspections: 0,
    };

    let monkey_1 = Monkey {
        idx: 1,
        items: vec![75, 70, 82, 83, 96, 64, 62],
        operation: |x| -> i64 { x + 8 },
        test: |x| divide_and_pick_monkey(x, 17, 5, 6),
        inspections: 0,
    };

    let monkey_2 = Monkey {
        idx: 2,
        items: vec![53],
        operation: |x| -> i64 { x * 3 },
        test: |x| divide_and_pick_monkey(x, 2, 0, 7),
        inspections: 0,
    };

    let monkey_3 = Monkey {
        idx: 3,
        items: vec![85, 64, 93, 64, 99],
        operation: |x| -> i64 { x + 4 },
        test: |x| divide_and_pick_monkey(x, 7, 7, 2),
        inspections: 0,
    };

    let monkey_4 = Monkey {
        idx: 4,
        items: vec![61, 92, 71],
        operation: |x| -> i64 { x * x },
        test: |x| divide_and_pick_monkey(x, 3, 2, 3),
        inspections: 0,
    };

    let monkey_5 = Monkey {
        idx: 5,
        items: vec![79, 73, 50, 90],
        operation: |x| -> i64 { x + 2 },
        test: |x| divide_and_pick_monkey(x, 11, 4, 6),
        inspections: 0,
    };

    let monkey_6 = Monkey {
        idx: 6,
        items: vec![50, 89],
        operation: |x| -> i64 { x + 3 },
        test: |x| divide_and_pick_monkey(x, 13, 4, 3),
        inspections: 0,
    };

    let monkey_7 = Monkey {
        idx: 7,
        items: vec![83, 56, 64, 58, 93, 91, 56, 65],
        operation: |x| -> i64 { x + 5 },
        test: |x| divide_and_pick_monkey(x, 19, 1, 0),
        inspections: 0,
    };

    vec![
        monkey_0, monkey_1, monkey_2, monkey_3, monkey_4, monkey_5, monkey_6, monkey_7,
    ]
}

fn divide_and_pick_monkey(x: i64, divide_value: i64, true_idx: u32, false_idx: u32) -> (u32, i64) {
    return if x % divide_value == 0 {
        (true_idx, x % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19))
    } else {
        (false_idx, x % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19))
    };
}
