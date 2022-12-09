//Before consolidating p1 and p2
use std::collections::BTreeSet;
use std::fs;

fn follow(h: &[i32; 2], t: &mut [i32; 2]) {
    if (h[0] - t[0]).abs() > 1 || (h[1] - t[1]).abs() > 1 {
        for i in 0..=1 {
            match h[i] - t[i] {
                1 | 2 => t[i] += 1,
                -2 | -1 => t[i] -= 1,
                _ => (),
            }
        }
    }
}

fn p1(lines: &Vec<&str>) {
    let mut head_pos: [i32; 2] = [0, 0];
    let mut tail_pos: [i32; 2] = [0, 0];

    let mut visited = BTreeSet::new();
    visited.insert(tail_pos);

    lines.iter().for_each(|l| {
        let mut tokens = l.split_ascii_whitespace();
        let (dir, amt) = (
            tokens.next().unwrap(),
            tokens.next().unwrap().parse::<i32>().unwrap(),
        );
        (0..amt).for_each(|_i| {
            match dir {
                "L" => head_pos[0] -= 1,
                "R" => head_pos[0] += 1,
                "D" => head_pos[1] -= 1,
                _ => head_pos[1] += 1,
            }
            follow(&head_pos, &mut tail_pos);
            visited.insert(tail_pos);
        });
    });

    println!("Part 1: {}", visited.len());
}

fn p2(lines: &Vec<&str>) {
    let mut knots = vec![[0, 0]; 10];

    let mut visited = BTreeSet::new();
    visited.insert(knots[9]);

    lines.iter().for_each(|l| {
        let mut tokens = l.split_ascii_whitespace();
        let (dir, amt) = (
            tokens.next().unwrap(),
            tokens.next().unwrap().parse::<i32>().unwrap(),
        );
        (0..amt).for_each(|_i| {
            match dir {
                "L" => knots[0][0] -= 1,
                "R" => knots[0][0] += 1,
                "D" => knots[0][1] -= 1,
                _ => knots[0][1] += 1,
            }
            (1..10).for_each(|j| {
                let (left, right) = knots[j - 1..j + 1].split_at_mut(1);
                follow(&left[0], &mut right[0]);
            });
            visited.insert(knots[9]);
        });
    });

    println!("Part 1: {}", visited.len());
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let lines = f.split("\n").collect::<Vec<&str>>();

    p1(&lines);
    p2(&lines);
}
