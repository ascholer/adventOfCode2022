use nom::{
    bytes::complete::{is_not, tag, take_till},
    character::complete::{digit1, space1},
    combinator::opt,
    multi::many1,
    sequence::{delimited, preceded, terminated},
    IResult,
};
use std::fs;

fn num_parse(input: &str) -> IResult<&str, u64> {
    let (i, num_str) = preceded(take_till(|c| char::is_ascii_digit(&c)), digit1)(input)?;
    Ok((i, num_str.parse::<u64>().unwrap()))
}

fn op_parse(input: &str) -> IResult<&str, &str> {
    let (i, op) = preceded(is_not("+*"), is_not(" "))(input)?;
    Ok((i, op))
}

fn num_list_parse(input: &str) -> IResult<&str, Vec<u64>> {
    let (i, res) = many1(delimited(
        take_till(|c| char::is_ascii_digit(&c)),
        digit1,
        opt(tag(",")),
    ))(input)?;
    let nums = res
        .iter()
        .map(|r| r.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    Ok((i, nums))
}

struct Monkey {
    items: Vec<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    divisor: u64,
    true_target: usize,
    false_target: usize,
    inspects: u64,
}

impl Monkey {
    fn inspect_results(&mut self, worry_cap: u64) -> Vec<(u64, usize)> {
        let results = self
            .items
            .drain(..)
            .map(|mut item| {
                self.inspects += 1;
                item = (self.op)(item) % worry_cap;
                let target_index = if item % self.divisor == 0 {
                    self.true_target
                } else {
                    self.false_target
                };
                (item, target_index)
            })
            .collect::<Vec<_>>();
        results
    }

    fn from_string(s: &str) -> Option<Monkey> {
        let mut lines = s.lines();
        lines.next(); //discard first

        let items = num_list_parse(lines.next()?).unwrap().1;

        let (operand, op_sym) = terminated(op_parse, space1)(lines.next()?).unwrap();
        let op_amt = operand.parse::<u64>().unwrap_or(0);
        let op_err = operand.parse::<u64>();
        let op: Box<dyn Fn(u64) -> u64> = if op_sym == "+" {
            Box::new(move |i| i + if op_err.is_err() { i } else { op_amt })
        } else {
            Box::new(move |i| i * if op_err.is_err() { i } else { op_amt })
        };

        let divisor = num_parse(lines.next()?).unwrap().1;
        let true_target = num_parse(lines.next()?).unwrap().1 as usize;
        let false_target = num_parse(lines.next()?).unwrap().1 as usize;

        Some(Monkey {
            items,
            op,
            divisor,
            true_target,
            false_target,
            inspects: 0,
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

    let worry_cap = monkeys
        .iter()
        .map(|m| m.divisor)
        .reduce(|acc, i| acc * i)
        .unwrap();

    for _r in 0..10000 {
        for mi in 0..monkeys.len() {
            for (item, target_index) in monkeys[mi].inspect_results(worry_cap) {
                monkeys[target_index].items.push(item)
            }
        }
    }

    let mut monkey_activity = monkeys.iter().map(|m| m.inspects).collect::<Vec<u64>>();
    monkey_activity.sort_by(|a, b| b.cmp(a));

    println!("Part 2: {}", monkey_activity[0] * monkey_activity[1])
}
