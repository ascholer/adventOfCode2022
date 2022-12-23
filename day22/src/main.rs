use std::fs;

use nom::combinator::opt;
use nom::multi::many1;
use nom::character::complete::{i32, alpha1, one_of};
use nom::sequence::pair;
use nom::{IResult, character::complete::anychar};

const GRID_SIZE : i32 = 50;

fn move_parse(input: &str) -> IResult<&str, Vec<(Option<char>, i32)>> {
    let (i, res) = many1(pair(opt(one_of("LR")), i32))(input)?; 
    Ok((i, res))
}


fn is_wall(grid : &Vec<String>, pos: &(i32, i32)) -> bool {
    grid[pos.0 as usize].chars().nth(pos.1 as usize).unwrap() == '#'
}

fn get_offset(dis:i32) -> i32 {
    (dis - 1) % GRID_SIZE
}

fn find_next_pos(grid : &Vec<String>, pos: &mut (i32, i32), dir: &mut (i32, i32)) {
    match dir.0 {
        0 => {  //going left or right
            let mut new_col = pos.1 + dir.1;
            let row = &grid[pos.0 as usize];
            let row_len = row.len() as i32;
            match row.chars().nth(new_col as usize).unwrap() {
                '#' => { return; },
                ' ' => {
                    //fun time...
                    if pos.0 <= GRID_SIZE {
                        //first grid row
                        if new_col == GRID_SIZE {
                            //left off 1... now going right in 5 if open
                            let offset = get_offset(pos.0);
                            let new_pos = (3 * GRID_SIZE - offset, 1);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (0, 1);
                            }
                            return;
                        } else if new_col == 3 * GRID_SIZE + 1 {
                            //right off 2 -> 4 going left
                            let offset = get_offset(pos.0);
                            let new_pos = (3 * GRID_SIZE - offset, 2 * GRID_SIZE);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (0, -1);
                            } 
                            return;
                        }
                        panic!("oops");
                    } else if pos.0 <= GRID_SIZE * 2 {
                        //second grid row
                        if new_col == GRID_SIZE {
                            //left off 3... now going down in 5 if open
                            let offset = get_offset(pos.0);
                            let new_pos = (2 * GRID_SIZE + 1, 1 + offset);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (1, 0);
                            }
                            return;
                        } else if new_col == 2 * GRID_SIZE + 1 {
                            //right off 3 -> 2 going up
                            let offset = get_offset(pos.0);
                            let new_pos = (GRID_SIZE, 2 * GRID_SIZE + 1 + offset);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (-1, 0);
                            }
                            return;
                        }
                        panic!("oops");
                    } else if pos.0 <= GRID_SIZE * 3 {
                        //third grid row
                        if new_col == 0 {
                            //left off 5... now going right in 1 if open
                            let offset = get_offset(pos.0);
                            let new_pos = (GRID_SIZE - offset, GRID_SIZE + 1);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (0, 1);
                            }
                            return;
                        } else if new_col == 2 * GRID_SIZE + 1 {
                            //right off 4 -> 2 going left
                            let offset = get_offset(pos.0);
                            let new_pos = (GRID_SIZE - offset, 3 * GRID_SIZE);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (0, -1);
                            }
                            return;
                        }
                        panic!("oops");
                    } else if pos.0 <= GRID_SIZE * 4 {
                        //fourth grid row
                        if new_col == 0 {
                            //left off 6... now going down in 1 if open
                            let offset = get_offset(pos.0);
                            let new_pos = (1, GRID_SIZE + 1 + offset);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (1, 0);
                            }
                            return;
                        } else if new_col == GRID_SIZE + 1 {
                            //right off 6 -> 4 going up
                            let offset = get_offset(pos.0);
                            let new_pos = (3 * GRID_SIZE, GRID_SIZE + 1 + offset);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (-1, 0);
                            }
                            return;
                        }
                        panic!("oops");
                    }
                    dbg!(pos);
                    panic!("oops");
                },
                _ => {      //. or moved
                    pos.1 = new_col; return;
                },
            }
        },
        _ => {  //going up or down
            let num_rows = grid.len() as i32;
            let mut new_row = pos.0 + dir.0;
            match grid[new_row as usize].chars().nth(pos.1 as usize).unwrap() {
                '#' => { return; },
                ' ' => {
                    //fun time...
                    if pos.1 <= GRID_SIZE {
                        //first grid col
                        if new_row == GRID_SIZE * 2 {
                            //up off 5 -> 3 going right
                            let offset = get_offset(pos.1);
                            let new_pos = (GRID_SIZE + 1 + offset, GRID_SIZE + 1);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (0, 1);
                            }
                            return;
                        } else if new_row == GRID_SIZE * 4 + 1 {
                            //down off 6 -> 2 going down
                            let offset = get_offset(pos.1);
                            let new_pos = (1, 2 * GRID_SIZE + 1 + offset);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (1, 0);
                            }
                            return;
                        }
                        panic!("oops");
                    } else if pos.1 <= 2 * GRID_SIZE {
                        //second grid col
                        if new_row == 0 {
                            //up off 1 -> 6 going right
                            let offset = get_offset(pos.1);
                            let new_pos = (3 * GRID_SIZE + 1 + offset, 1);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (0, 1);
                            }
                            return;
                        } else if new_row == GRID_SIZE * 3 + 1 {
                            //down off 4 -> 6 going left
                            let offset = get_offset(pos.1);
                            let new_pos = (3 * GRID_SIZE + 1 + offset, GRID_SIZE);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (0, -1);
                            }
                            return;
                        }
                        panic!("oops");
                    } else if pos.1 <= 3 * GRID_SIZE {
                        //third grid col
                        if new_row == 0 {
                            //up off 2 -> 6 going up
                            let offset = get_offset(pos.1);
                            let new_pos = (4 * GRID_SIZE, 1 + offset);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (-1, 0);
                            }
                            return;
                        } else if new_row == GRID_SIZE + 1 {
                            //down off 2 -> 3 going left
                            let offset = get_offset(pos.1);
                            let new_pos = (GRID_SIZE + 1 + offset, 2 * GRID_SIZE);
                            if !is_wall(grid, &new_pos) {
                                (pos.0, pos.1) = (new_pos.0, new_pos.1);
                                (dir.0, dir.1) = (0, -1);
                            }
                            return;
                        }
                        panic!("oops");
                    }
                    panic!("oops");
                },
                _ => {      //dor or move
                    pos.0 = new_row; return;
                },
            }
        }
    }
}

fn turn(dir: &mut (i32, i32), turn : char) {
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
}

fn print_grid(grid : &Vec<String>) {
    for r in grid {
        for c in r.chars() {
            print!("{}", c);
        }
        println!("");
    }
}

fn do_move(grid : &mut Vec<String>, pos: &mut (i32, i32), dir: &mut (i32, i32), m : &(Option<char>, i32)) {
    match m.0 {
        Some(turn_dir) => {
            turn(dir, turn_dir);
        },
        _ => {}
    }
    println!("Dir {:?} moves: {}", &dir, &m.1);
    for s in 0..m.1 {
        find_next_pos(grid, pos, dir);
        grid[pos.0 as usize].replace_range((pos.1 as usize)..((pos.1+1) as usize), "X");
        print!("{:?} ", &pos);
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
    let pad = " ".repeat(max_len + 2);

    //place spaces all around to find wraps easier
    for i in 0..grid.len() {
        grid[i] = " ".to_owned() + &grid[i];
        let cur_len = grid[i].len();
        grid[i].push_str(&pad[0..(max_len + 2 - cur_len)]);
    }
    grid.insert(0,pad.clone());
    grid.push(pad.clone());

    let mut pos = (1, grid[1].chars().position(|c| c == '.').unwrap() as i32);
    let mut dir = (0, 1);
    for m in &moves {
        do_move(&mut grid, &mut pos, &mut dir, &m);
        println!(" ");
        //print_grid(&grid);
    }

    let face_val = match dir {
        (0,1) => 0,
        (1,0) => 1,
        (0,-1) => 2,
        _ => 3
    };
    println!("{:?}", &pos);

    let sum = 1000 * (pos.0) + 4 * (pos.1) + face_val;
    println!("Part2 : {}", sum);
}
