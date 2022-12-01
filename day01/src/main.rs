use std::fs;

fn main() -> std::io::Result<()> {
    let s = fs::read_to_string("data.txt").unwrap();
    let elves = s.split("\n\n").map(|l| l.split("\n").map(|s| str::parse::<i32>(s).unwrap()));
    let mut elf_totals: Vec<i32> = elves.map(|e| e.sum::<i32>()).collect();
    elf_totals.sort_by(|a, b| b.cmp(a));
    
    println!("Part 1: {:?}", elf_totals[0]);

    let total = elf_totals[0..3].iter().sum::<i32>();
    println!("Part 2: {:?}", total);

    Ok(())
}
