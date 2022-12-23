use std::fs;

use nom::combinator::opt;
use nom::multi::many1;
use nom::character::complete::{i32, alpha1, one_of};
use nom::sequence::pair;
use nom::{IResult, character::complete::anychar};

fn move_parse(input: &str) -> IResult<&str, Vec<(Option<char>, i32)>> {
    let (i, res) = many1(pair(opt(one_of("LR")), i32))(input)?; 
    Ok((i, res))
}

fn find_next_pos(grid : &Vec<String>, pos: &mut (i32, i32), dir: &(i32, i32)) {
    match dir.0 {
        0 => {  //going left or right
            let mut new_col = pos.1 + dir.1;
            let row = &grid[pos.0 as usize];
            let row_len = row.len() as i32;
            new_col = (new_col + row_len) % row_len;
            while row.chars().nth(new_col as usize).unwrap() == ' ' {
                new_col += dir.1;
                new_col = (new_col + row_len) % row_len;
            }
            match row.chars().nth(new_col as usize).unwrap() {
                '.' => { pos.1 = new_col },
                '#' => { },
                _ => {      //ruhroh
                    panic!("Crap");
                },
            }
        },
        _ => {  //going up or down
            let num_rows = grid.len() as i32;
            let mut new_row = pos.0 + dir.0;
            new_row = (new_row + num_rows) % num_rows;

            while grid[new_row as usize].chars().nth(pos.1 as usize).unwrap() == ' ' {
                new_row += dir.0;
                new_row = (new_row + num_rows) % num_rows;
            }
            match grid[new_row as usize].chars().nth(pos.1 as usize).unwrap() {
                '.' => { pos.0 = new_row },
                '#' => { },
                _ => {      //ruhroh
                    panic!("Crap");
                },
            }
        }
    }
}

fn do_move(grid : &Vec<String>, pos: &mut (i32, i32), dir: &mut (i32, i32), m : &(Option<char>, i32)) {
    match m.0 {
        Some(turn) => {
            match turn {
                'R' => {
                    let temp = dir.0;
                    dir.0 = dir.1;
                    dir.1 = -temp;
                },
                _ => {
                    let temp = dir.0;
                    dir.0 = -dir.1;
                    dir.1 = temp;
                }
            }
        },
        _ => {}
    }
    for s in 0..m.1 {
        find_next_pos(grid, pos, dir);
    }
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let lines = f.lines();
    let moves = move_parse(lines.last().unwrap()).unwrap().1;

    let mut grid : Vec<String> = Vec::new();
    for l in f.lines() {
        if l.len() == 0 {
            break;
        }
        grid.push(l.to_string());
    }
    let max_len = grid.iter().map(|r| r.len()).max().unwrap();
    let pad = " ".repeat(max_len);
    for mut r in &mut grid {
        r.push_str(&pad[0..(max_len - r.len())]);
    }

    let mut pos = (0, grid[0].chars().position(|c| c == '.').unwrap() as i32);
    let mut dir = (0, 1);
    for m in &moves {
        do_move(&grid, &mut pos, &mut dir, &m);
    }

    let face_val = match dir {
        (0,1) => 0,
        (1,0) => 1,
        (0,-1) => 2,
        _ => 3
    };
    
    let sum = 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + face_val;
    println!("Part1 : {}", sum);
}
