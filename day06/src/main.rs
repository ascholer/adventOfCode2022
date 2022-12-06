use std::fs;

fn find_marker(sequence: &str, length: usize) -> usize {
    for i in 0.. {
        let slice = &sequence[i..i + length];
        if slice.chars().all(|c| slice.matches(c).count() == 1) {
            return i + length;
        }
    }
    0
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("data.txt").unwrap();

    println!("Part 1: {:?}", find_marker(&input, 4));
    println!("Part 2: {:?}", find_marker(&input, 14));

    Ok(())
}
