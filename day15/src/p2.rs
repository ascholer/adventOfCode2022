use std::cmp::max;
use std::fs;

use nom::character::complete::i32;
use nom::sequence::{delimited, preceded};
use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::opt, multi::many1,
    sequence::separated_pair, IResult,
};

#[derive(Clone, Copy)]
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
    use std::time::Instant;
    let now = Instant::now();

    const CHECK_MAX: i32 = 4000000;
    let f = fs::read_to_string("data.txt").unwrap();

    let loc_pairs = line_parse(&f).unwrap().1;

    let sensors: Vec<Sensor> = loc_pairs
        .iter()
        .map(|p| Sensor {
            loc: p.0,
            exclude_range: dist(&p.0, &p.1),
        })
        .collect();

    let mut found = false;
    let mut found_loc = Loc { x: -1, y: -1 };
    let mut x = 0;
    while x <= CHECK_MAX && !found {
        let mut y = 0;
        while y <= CHECK_MAX {
            let cur_loc = Loc { x, y };
            let mut next_y = y;

            for s in &sensors {
                let d = dist(&s.loc, &cur_loc);
                if d <= s.exclude_range {
                    let x_dif = (s.loc.x - cur_loc.x).abs();
                    next_y = s.loc.y + (s.exclude_range - x_dif) + 1;
                    break;
                    //quicker by significant degree to break at first match
                    //instead of looking for largest possible jump
                }
            }

            if y == next_y {
                found = true;
                found_loc = cur_loc;
                break;
            } else {
                y = next_y;
            }
        }
        x += 1;
    }

    if found {
        println!(
            "Part 2: {}",
            found_loc.x as i128 * 4000000 + found_loc.y as i128
        );
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    } else {
        println!("Ruh roh");
    }
}
