use regex::Regex;
use std::fs;

struct Monkey {
    items: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    divisor: i64,
    true_target: usize,
    false_target: usize,
    inspects: i64
}

impl Monkey {
    fn from_string(s: &str) -> Option<Monkey> {
        let re_items: Regex = Regex::new(r#"(\d+)"#).ok()?;
        let re_op: Regex = Regex::new(r#" ([\*\+]) (\w+)"#).ok()?;
        let re_num: Regex = Regex::new(r#"(\d+)"#).ok()?;
        let mut lines = s.lines();
        lines.next(); //discard first
        let items = re_items.find_iter(lines.next()?);
        let items: Vec<i64> = items.map(|i| i.as_str().parse::<i64>().unwrap()).collect();
        let op_cap = re_op.captures(lines.next()?)?;
        let (op_sym, operand) = (
            op_cap.get(1)?.as_str(),
            op_cap.get(2)?.as_str(),
        );
        let op_amt = operand.parse::<i64>().unwrap_or(0);
        let op_err = operand.parse::<i64>();
        let op: Box<dyn Fn(i64) -> i64> = if op_sym == "+" {
            Box::new(move |i| i + if op_err.is_err() {i} else {op_amt})
        } else {
            Box::new(move |i| i * if op_err.is_err() {i} else {op_amt})
        };
        let divisor = re_num
            .captures(lines.next().unwrap())?
            .get(1)?
            .as_str()
            .parse::<i64>().unwrap();
        let true_target = re_num
            .captures(lines.next().unwrap())?
            .get(1)?
            .as_str()
            .parse::<usize>()
            .unwrap();
        let false_target = re_num
            .captures(lines.next().unwrap())?
            .get(1)?
            .as_str()
            .parse::<usize>()
            .unwrap();
        Some(Monkey {
            items,
            op,
            divisor,
            true_target,
            false_target,
            inspects: 0
        })
    }
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let monkey_strings = f.split("\n\n");
    let mut monkeys: Vec<Monkey> = Vec::new();
    monkey_strings.for_each(|s| {
        let m = Monkey::from_string(s).unwrap();
        monkeys.push(m);
    });

    for r in 0..20 {
        for mi in 0..monkeys.len() {
            let (before, rest) = monkeys[0..].split_at_mut(mi);
            let (m_it, after) = rest[0..].split_at_mut(1);
            let m = &mut m_it[0];
            for mut item in m.items.drain(0..) {
                m.inspects += 1;
                item = (m.op)(item) / 3;
                let target_index = if item % m.divisor == 0 {
                    m.true_target
                } else {
                    m.false_target
                };

                let target = if target_index < mi {
                    &mut before[target_index]
                } else {
                    &mut after[target_index - mi - 1]
                };

                target.items.push(item);
            }
        }
    }

    let mut monkey_activity = monkeys.iter().map(|m| m.inspects).collect::<Vec<i64>>();
    monkey_activity.sort_by(|a,b| b.cmp(a));
    println!("Part 1: {}", monkey_activity[0] * monkey_activity[1])

}
