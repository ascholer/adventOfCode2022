use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till},
    character::complete::{alpha1, anychar, digit1, multispace1, space1},
    combinator::opt,
    error::Error,
    multi::many1,
    number::complete::be_i64,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use nom::number::complete::i64;

use std::{collections::HashMap, fs, hash::Hash};

enum Monkey {
    ValueMonkey(ValueMonkey),
    OpMonkey(OpMonkey),
}

struct ValueMonkey {
    name: String,
    val: i64,
}
struct OpMonkey {
    name: String,
    op: char,
    operand1: String,
    operand2: String,
}

fn op_parse(input: &str) -> IResult<&str, (&str, char, &str)> {
    let (i, op) = tuple((alpha1, delimited(multispace1, anychar, multispace1), alpha1))(input)?;
    Ok((i, op))
}

fn monkey_parse(input: &str) -> IResult<&str, &str> {
    let (i, op) = terminated(alpha1, alpha1)(input)?;
    Ok((i, op))
}

impl Monkey {
    fn value(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match self {
            Monkey::OpMonkey(m) => {
                let m1_val = monkeys[&m.operand1].value(monkeys);
                let m2_val = monkeys[&m.operand2].value(monkeys);
                match m.op {
                    '+' => m1_val + m2_val,
                    '-' => m1_val - m2_val,
                    '*' => m1_val * m2_val,
                    _ => m1_val / m2_val,
                }
            }
            Monkey::ValueMonkey(m) => m.val,
        }
    }

    fn from_string(s: &str) -> Option<Monkey> {
        let (s, name) = terminated::<_, _, _, Error<_>, _, _>(alpha1, tag(": "))(s).unwrap();
        if s.chars().nth(0).unwrap().is_numeric() {
            let num = s.parse::<i64>().unwrap();
            return Some(Monkey::ValueMonkey(ValueMonkey {
                name: name.to_owned(),
                val: num,
            }));
        } else {
            let (s, op_parts) = op_parse(s).unwrap();
            return Some(Monkey::OpMonkey(OpMonkey {
                name: name.to_owned(),
                op: op_parts.1,
                operand1: op_parts.0.to_string(),
                operand2: op_parts.2.to_string(),
            }));
        }
    }

    fn get_name(&self) -> &String {
        match self {
            Monkey::OpMonkey(m) => &m.name,
            Monkey::ValueMonkey(m) => &m.name,
        }
    }

    fn set_value(&mut self, v: i64) {
        match *self {
            Monkey::ValueMonkey(ref mut m) => m.val = v,
            _ => {}
        }
    }
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();

    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    for l in f.lines() {
        let m = Monkey::from_string(l).unwrap();
        monkeys.insert(m.get_name().clone(), m);
    }

    let target = monkeys["bmgf"].value(&monkeys);

    let mut min = 0;
    let mut max = 6000000000000;
    while min <= max {
        let mid = (min + max) / 2;
        monkeys.get_mut("humn").unwrap().set_value(mid);
        let cur = monkeys["sbtm"].value(&monkeys);
        println!("{} : {}", mid, cur);
        if cur > target {
            min = mid + 1;
        } else if cur == target {
            println!("Part2 : {}", mid);
            break;
        } else {
            max = mid - 1;
        }
    }
}
