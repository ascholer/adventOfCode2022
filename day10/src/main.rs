use odds::string::StrChunksWindows;

use std::fs;

fn main() {
    //add phantom noop to offset states by 1
    let f = "noop\n".to_string() + &fs::read_to_string("data.txt").unwrap();

    let states = f
        .lines()
        //each line becomes a list of deltas that are flattened into one list
        .flat_map(|l| {
            let mut tokens = l.split_ascii_whitespace();
            match tokens.next().unwrap() {
                "addx" => {
                    //add produces no change, then given change
                    vec![0, tokens.next().unwrap().parse::<i32>().unwrap()]
                }
                //noop one state
                _ => vec![0],
            }
        })
        //deltas => curx
        .scan(1, |curx, d| {
            *curx = *curx + d;
            Some(*curx)
        })
        //to (clock, time)
        .scan((0, 0), |state, curx| {
            state.0 += 1;
            state.1 = curx;
            Some(*state)
        })
        .collect::<Vec<_>>();

    let sum = states
        .iter()
        .filter(|p| (p.0 as i32) % 40 == 20)
        .map(|p| (p.0 as i32) * p.1)
        .sum::<i32>();
    println!("Part 1: {}", sum);

    println!("Part 2:");
    states
        .iter()
        .map(|p| {
            let clock = (p.0 as i32) % 40;
            if (clock - p.1 - 1).abs() < 2 {
                '#'
            } else {
                '.'
            }
        })
        .collect::<String>()
        .char_chunks(40)
        .for_each(|s| println!("{:?}", s));
}
