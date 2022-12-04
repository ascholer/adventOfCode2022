//Requires build with +nightly
#![feature(iter_array_chunks)]

use std::{fs};

fn value(c : char) -> i32 {
    if c <= 'Z' {
        return i32::try_from(c as u32 - 'A' as u32).unwrap() + 27;
    }
    return i32::try_from(c as u32 - 'a' as u32).unwrap() + 1;
}

fn p1() {
    let file = fs::read_to_string("data.txt").unwrap();
    let sacks = file.split("\n");

    let mut sum = 0;
    for s in sacks {
        let h1 = &s[0..s.len()/2];
        let h2 = &s[s.len()/2..];
        let dupe = h1.chars().filter(|c| h2.contains(*c)).collect::<Vec<char>>()[0];
        sum += value(dupe);
    }
    println!("Part 1: {:?}", sum);
}

fn p2() {
    let file = fs::read_to_string("data.txt").unwrap();
    let lines = file.split("\n").into_iter();

    let groups = lines.array_chunks::<3>(); 
    let mut sum = 0;
    for g in groups {
        let common_char = g[0].chars().filter(|c| g[1].contains(*c) && g[2].contains(*c)).next().unwrap();
        sum += value(common_char);
    }
    println!("Part 2: {:#?}", sum);

}

fn main() -> std::io::Result<()> {
    //Requires build with +nightly
    p1();
    p2();
    Ok(())
}
