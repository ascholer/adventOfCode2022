use std::fs::File;
use std::io::{BufRead, BufReader};


fn shift_char(c: char) -> char {
    match c {
        'X' => return 'A',
        'Y' => return 'B',
        _ => return 'C'
    }
}

fn beats(c: char,)  -> char {
    match c {
        'B' => return 'A',
        'C' => return 'B',
        _ => return 'C'
    }
}

fn beaten_by(c: char,)  -> char {
    match c {
        'A' => return 'B',
        'B' => return 'C',
        _ => return 'A'
    }
}

fn choose_move(p: char, goal: char)  -> char {
    match goal {
        'X' => return beats(p),
        'Y' => return p,
        _ => return beaten_by(p)
    }
}

fn round_score(p1: char, p2: char) -> i32 {
    let choice_points = match p2 {
        'A' => 1,
        'B' => 2,
        _ => 3,
    };
    let mut result_points = 0; //assume loss
    if p1 == p2 {
        result_points = 3;
    } else if beaten_by(p1) == p2 {
        result_points = 6;
    }
    choice_points + result_points
}

fn main() -> std::io::Result<()> {
    let lines = BufReader::new(File::open("data.txt")?).lines();

    let mut pairs : Vec<(char, char)> = Vec::new();
    for line in lines {
        let parts = line.unwrap();
        let mut parts = parts.split(" ").map(|s| s.chars().nth(0).unwrap());
        let tup = (parts.next().unwrap(), parts.next().unwrap());
        pairs.push(tup);
    }

    let sum1 = pairs.iter().map(|p| round_score(p.0, shift_char(p.1))).sum::<i32>();
    let sum2 = pairs.iter().map(|p| round_score(p.0, choose_move(p.0, p.1))).sum::<i32>();

    println!("Part 1: {:?}", sum1);
    println!("Part 2: {:?}", sum2);

    Ok(())
}
