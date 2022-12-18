use std::{fs, collections::VecDeque};

const BUF_SIZE : usize = 5000;

#[derive(Clone)]
struct Rock {
    grid : Vec<Vec<char>>,
    v_size : usize,
    h_size : usize,
    position : (usize, usize)
}

fn blow(r : &mut Rock, land : &Vec<VecDeque<char>>, dir : char) {
    if dir == '>' {
        let can_move = r.position.0 + r.h_size < 7 && !touch(r, land, (1, 0));
        if can_move {
            r.position.0 += 1;
        }
    } else {
        let can_move = r.position.0 > 0 && !touch(r, land, (-1, 0));
        if can_move {
            r.position.0 -= 1;
        }
    }
}

fn touch(r : &Rock, land : &Vec<VecDeque<char>>, offset : (i32, i32)) -> bool {
    for i in 0..r.grid.len() {
        for j in 0..r.grid.len() {
            if r.grid[i][j] != 'X' {
                continue;
            }
            let col : i32 = (r.position.0 + j) as i32 + offset.0;
            let row : i32 = (r.position.1 - i) as i32 + offset.1;
            if land[col as usize][row as usize] != '.' {
                return true;
            }
        }
    }

    false
}


fn cleanup(land : &mut Vec<VecDeque<char>>, heights : &mut [usize; 7]) -> usize {
    let chop_height = *heights.iter().min().unwrap();
    for col in 0..7 {
        drop(land[col].drain(0..chop_height));
        land[col].resize(BUF_SIZE, '.');
        heights[col] -= chop_height;
    }
    chop_height
}


fn main() {    
    use std::time::Instant;
    let now = Instant::now();
    let jets = fs::read_to_string("data.txt").unwrap();
    let mut jet_pos = 0;

    //[col][height]
    let mut land = vec![VecDeque::from_iter(['.'; 100]); 7];
    for c in 0..7 {
        land[c][0] = '-';
    }
    
    let rocks = [
        Rock {
            grid : vec![vec!['X'; 4],vec!['.'; 4],vec!['.'; 4],vec!['.'; 4] ],
            v_size: 1,
            h_size: 4,
            position : (0,0)
        },
        Rock {
            grid : vec![vec!['.', 'X', '.', '.'],vec!['X', 'X', 'X', '.'],vec!['.', 'X', '.', '.'],vec!['.'; 4] ],
            v_size: 3,
            h_size: 3,
            position : (0,0)
        },
        Rock {
            grid : vec![vec!['.', '.', 'X', '.'],vec!['.', '.', 'X', '.'],vec!['X', 'X', 'X', '.'],vec!['.'; 4] ],
            v_size: 3,
            h_size: 3,
            position : (0,0)
        },
        Rock {
            grid : vec![vec!['X', '.', '.', '.']; 4],
            v_size: 4,
            h_size: 1,
            position : (0,0)
        },
        Rock {
            grid : vec![vec!['X', 'X', '.', '.'], vec!['X', 'X', '.', '.'], vec!['.', '.', '.', '.'], vec!['.', '.', '.', '.']],
            v_size: 2,
            h_size: 2,
            position : (0,0)
        },
    ];

    let mut heights = [0 as usize; 7];
    let mut total_chop = 0;
    let mut last_even = 0;
    for i in 0..1875 {
        if i % 100 == 0 { 
            let chop = cleanup(&mut land, &mut heights);
            total_chop += chop;
        }

        if i > 0 && heights.iter().all(|h| h == &heights[0]) {
            let chop = cleanup(&mut land, &mut heights);
            total_chop += chop;
            println!("Even at {} - diff {}", i, i - last_even);
            last_even = i;
            let elapsed = now.elapsed();
            println!("Chop total {}", total_chop);
            println!("{} - Elapsed: {:.2?}", i, elapsed);
        }

        let mut r = rocks[i % 5].clone();

        let max_height = heights.iter().max().unwrap();
        r.position = (2, r.v_size + 3 + max_height);
        let mut hit_floor = false;
        while !hit_floor {
            let jet_dir = jets.chars().nth(jet_pos).unwrap();
            blow(&mut r, &land, jet_dir);
            jet_pos = (jet_pos + 1) % jets.chars().count();
            hit_floor = touch(&r, &land, (0, -1));
            if !hit_floor {
                r.position.1 -= 1;
            }
        }

        for i in 0..r.grid.len() {
            for j in 0..r.grid.len() {
                if r.grid[i][j] != 'X' {
                    continue;
                }
                let col = j + r.position.0;
                let row = (r.position.1 - i) as usize;
                if row > heights[col] {
                    heights[col] = row;
                }
                land[col][row] = 'X';
            }
        }
    }
    
    let max_height = land.iter().map(|v| {
        v.iter().rposition(|item| item != &'.').unwrap()
    }).max().unwrap();

    let bonus = 2673 as usize * 576368875 as usize;

    println!("Part 2: {}", max_height + total_chop + bonus);

}
