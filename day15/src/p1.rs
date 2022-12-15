use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

use nom::character::complete::i32;
use nom::sequence::{delimited, preceded};
use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::opt, multi::many1,
    sequence::separated_pair, IResult,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Loc {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Sensor {
    loc: Loc,
    exclude_range: i32,
}

fn xy_pair(input: &str) -> IResult<&str, Loc> {
    let (i, res) = preceded(opt(tag("x=")), separated_pair(i32, tag(", y="), i32))(input)?;
    Ok((i, Loc { x: res.0, y: res.1 }))
}

fn line_parse(input: &str) -> IResult<&str, Vec<(Loc, Loc)>> {
    let (i, res) = many1(delimited(
        tag("Sensor at "),
        separated_pair(xy_pair, tag(": closest beacon is at "), xy_pair),
        opt(line_ending),
    ))(input)?;
    Ok((i, res))
}

fn dist(a: &Loc, b: &Loc) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn main() {
    const CHECK_Y: i32 = 2000000;

    let f = fs::read_to_string("data.txt").unwrap();

    let loc_pairs = line_parse(&f).unwrap().1;

    let sensors: Vec<Sensor> = loc_pairs
        .iter()
        .map(|p| Sensor {
            loc: p.0,
            exclude_range: dist(&p.0, &p.1),
        })
        .collect();

    let beacons = loc_pairs.iter().map(|p| p.1).collect::<HashSet<_>>();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    for s in &sensors {
        min_x = min(min_x, s.loc.x - s.exclude_range);
        max_x = max(max_x, s.loc.x + s.exclude_range);
    }

    let mut count = 0;
    for x in min_x..=max_x {
        if sensors
            .iter()
            .any(|s| dist(&s.loc, &Loc { x, y: CHECK_Y }) <= s.exclude_range)
        {
            count += 1;
        }
    }

    count -= beacons.iter().filter(|b| b.y == CHECK_Y).count();

    println!("Part 1: {}", count);
}
