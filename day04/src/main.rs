use std::cmp;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    fn overlap(&self, other: &Range) -> bool {
        let max_start = cmp::max(self.start, other.start);
        let min_end = cmp::min(self.end, other.end);
        if min_end >= max_start {
            true
        } else {
            false
        }
    }
}

impl FromStr for Range {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once('-').unwrap();
        let sval = x.parse::<i32>()?;
        let eval = y.parse::<i32>()?;
        Ok(Range {
            start: sval,
            end: eval,
        })
    }
}

fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("data.txt").unwrap();
    let lines = file.split("\n");
    let range_pairs = lines.map(|s| {
        s.split(",")
            .map(|p| Range::from_str(&str::to_owned(p)).unwrap())
            .collect::<Vec<_>>()
    });

    let contains_count = range_pairs
        .clone()
        .filter(|p| p[0].contains(&p[1]) || p[1].contains(&p[0]))
        .count();
    println!("Part 1: {:?}", contains_count);

    let overlaps = range_pairs.filter(|p| p[0].overlap(&p[1])).count();
    println!("Part 2: {:?}", overlaps);
    Ok(())
}
