use std::fs;

fn main() -> std::io::Result<()> {
    let s = fs::read_to_string("data.txt").unwrap();
    let elves = s.split("\n\n").map(|l| l.split("\n"));
    let elves = elves.map(|e| e.map(|s| str::parse::<i32>(s).unwrap()));
    let mut elves: Vec<i32> = elves.map(|e| e.sum::<i32>()).collect();
    elves.sort_by(|a, b| b.cmp(a));
    let total = elves[0..3].iter().sum::<i32>();

    println!("{:?}", total);

    Ok(())
}
