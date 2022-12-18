use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

use nom::character::complete::i32;
use nom::sequence::terminated;
use nom::IResult;
use nom::{bytes::complete::tag, combinator::opt, multi::many1};

type Cube = [i32; 3];
const upper_bound: i32 = 22;

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

fn is_contained(c: &Cube, rocks: &HashSet<Cube>, explored: &mut HashSet<Cube>) -> bool {
    if explored.contains(c) || rocks.contains(c) {
        return true;
    }
    for dim in 0..=2 {
        if c[dim] < 0 || c[dim] > upper_bound {
            return false;
        }
    }
    explored.insert(*c);
    get_neighbors(c)
        .iter()
        .all(|n| is_contained(n, rocks, explored))
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let mut rocks: HashSet<Cube> = HashSet::new();
    for l in f.lines() {
        let rock = line_parse(&l).unwrap().1;
        rocks.insert(rock);
    }

    for x in 0..upper_bound {
        for y in 0..upper_bound {
            for z in 0..upper_bound {
                let mut contained: HashSet<Cube> = HashSet::new();
                let res = is_contained(&[x, y, z], &rocks, &mut contained);
                if res {
                    rocks.extend(contained);
                }
            }
        }
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
    println!("Part 2: {:?}", exposed);
}
