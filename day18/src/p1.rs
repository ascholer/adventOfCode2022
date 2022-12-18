use std::collections::HashSet;
use std::fs;

use nom::IResult;
use nom::character::complete::i32;
use nom::sequence::terminated;
use nom::{bytes::complete::tag, combinator::opt, multi::many1};

type Cube = [i32; 3];

fn get_neighbors(c: &Cube) -> Vec<Cube> {
    (0..=2)
        .flat_map(|dim| {
            (-1..=1).step_by(2).map(move |amt| {
                let mut n = c.clone();
                n[dim] += amt;
                n
            })
        })
        .collect()
}

fn line_parse(input: &str) -> IResult<&str, Cube> {
    let (i, res) = many1(terminated(i32, opt(tag(","))))(input)?;

    let res = [res[0], res[1], res[2]];

    Ok((i, res))
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let mut rocks: HashSet<Cube> = HashSet::new();
    for l in f.lines() {
        let rock = line_parse(&l).unwrap().1;
        rocks.insert(rock);
    }

    let exposed: usize = rocks
        .iter()
        .map(|r| {
            get_neighbors(r)
                .iter()
                .filter(|n| !rocks.contains(*n))
                .count()
        })
        .sum();
    println!("Part 1: {}", exposed);
}
