use std::collections::BTreeSet;
use std::fs;

struct Command {
    dir: char,
    amt: i32
}

fn follow(head: &[i32], tail: &mut [i32]) {
    if (head[0] - tail[0]).abs() > 1 || (head[1] - tail[1]).abs() > 1 {
        for i in 0..=1 {
            match head[i] - tail[i] {
                1 | 2 => tail[i] += 1,
                -2 | -1 => tail[i] -= 1,
                _ => (),
            }
        }
    }
}

fn simulate(commands: &Vec<Command>, num_tails: usize) -> usize {
    let mut knots = vec![[0, 0]; num_tails + 1];

    let mut visited = BTreeSet::new();
    visited.insert(knots[0]);

    commands.iter().for_each(|c| {
        (0..c.amt).for_each(|_i| {
            match c.dir {
                'L' => knots[0][0] -= 1,
                'R' => knots[0][0] += 1,
                'D' => knots[0][1] -= 1,
                _ => knots[0][1] += 1,
            }
            (1..knots.len()).for_each(|j| {
                let (left, right) = knots[j - 1..j + 1].split_at_mut(1);
                follow(&left[0], &mut right[0]);
            });
            visited.insert(knots[num_tails]);
        });
    });

    visited.len()
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let lines : Vec<Command> = f
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| {
            let mut tokens = l.split_ascii_whitespace();
            Command {
                dir: tokens.next().unwrap().chars().nth(0).unwrap(),
                amt: tokens.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect();

    println!("Part 1: {}", simulate(&lines, 1));
    println!("Part 2: {}", simulate(&lines, 9));
}
