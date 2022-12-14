use std::cmp::Ordering::{Greater, Less};
use std::{cmp::max, collections::HashMap, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::opt,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};

fn rocks_parse(input: &str) -> IResult<&str, Vec<Vec<[i32; 2]>>> {
    let (i, res) = many1(terminated(
        many1(terminated(
            separated_pair(digit1, tag(","), digit1),
            opt(tag(" -> ")),
        )),
        opt(line_ending),
    ))(input)?;

    let list = res
        .iter()
        .map(|line| {
            line.iter()
                .map(|p| [p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap()])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok((i, list))
}

fn drop_sand(map: &mut HashMap<[i32; 2], char>, max_depth: i32) -> bool {
    let mut sand_loc = [500, 0];
    while sand_loc[1] < max_depth {
        if !map.contains_key(&[sand_loc[0], sand_loc[1] + 1]) {
            sand_loc[1] += 1; //falls down
        } else {
            if !map.contains_key(&[sand_loc[0] - 1, sand_loc[1] + 1]) {
                sand_loc[0] -= 1;
                sand_loc[1] += 1; //fall left
            } else {
                if !map.contains_key(&[sand_loc[0] + 1, sand_loc[1] + 1]) {
                    sand_loc[0] += 1;
                    sand_loc[1] += 1; //fall right
                } else {
                    map.insert(sand_loc, 'o');
                    return true; //lands
                }
            }
        }
    }
    false
}

fn make_wall(map: &mut HashMap<[i32; 2], char>, mut start: [i32; 2], end: [i32; 2]) {
    let d = if start[0] != end[0] { 0 } else { 1 };
    while start[d] != end[d] {
        map.insert(start, '#');

        start[d] += match start[d].cmp(&end[d]) {
            Greater => -1,
            Less => 1,
            _ => 0,
        };
    }
    map.insert(end, '#');
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let rock_lines = rocks_parse(&f).unwrap().1;

    let mut map: HashMap<[i32; 2], char> = HashMap::new();

    let mut maxy = i32::MIN;

    rock_lines.iter().for_each(|line| {
        line[0..].windows(2).for_each(|pair| {
            let start = pair[0];
            let end = pair[1];
            maxy = max(maxy, end[1]);
            make_wall(&mut map, start, end);
        });
    });

    make_wall(&mut map, [0, maxy + 2], [1000, maxy + 2]);

    let mut count = 0;
    while !map.contains_key(&[500, 0]) {
        drop_sand(&mut map, maxy + 3);
        count += 1;
    }
    println!("Part2: {}", count);
}
