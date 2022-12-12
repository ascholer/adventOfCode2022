
use std::{fs, collections::{HashMap, VecDeque}, cmp::min};

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Loc {
    row : usize,
    col : usize,
}

impl Loc {
    fn new() -> Loc {
        Loc { row: 0, col: 0}
    }
    fn get_neighbors(&self, row_bound : usize, col_bound: usize) -> Vec<Loc> {
        let mut n : Vec<Loc> = Vec::new();
        if self.row > 0 { n.push( Loc {row: self.row - 1, col: self.col} )}
        if self.row < row_bound - 1 { n.push( Loc {row: self.row + 1, col: self.col} )}
        if self.col > 0 { n.push( Loc {row: self.row, col: self.col - 1} )}
        if self.col < col_bound - 1 { n.push( Loc {row: self.row, col: self.col + 1} )}
        n
    }
}

fn best_path(start : &Loc, end : &Loc, grid : &Vec<Vec<usize>>) -> i32 {
    let mut parent : HashMap<Loc, Loc> = HashMap::new();
    parent.insert(start.clone(), start.clone());
    let mut to_search : VecDeque<Loc> = VecDeque::new();

    to_search.push_back(start.clone());
    while to_search.len() > 0 {
        let cur = to_search.pop_front().unwrap();
        let neighbors = cur.get_neighbors(grid.len(), grid[0].len());
        let cur_height = grid[cur.row][cur.col];
        neighbors.iter().filter(|n| grid[n.row][n.col] <= cur_height + 1).for_each(|n| {
            if !parent.contains_key(&n) {
                parent.insert(n.clone(), cur.clone());
                to_search.push_back(n.clone());
            }
        });
    }
    let mut length = 0;
    let mut cur = end;
    while cur != start {
        length += 1;
        let p = parent.get(&cur);
        match p {
            Some(x) => cur = x,
            _ => return i32::MAX
        }
    }
    length
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let data = f.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut start : Loc = Loc::new();
    let mut end : Loc = Loc::new();
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            match data[i][j] {
                'S' => start = Loc {row: i, col: j},
                'E' => end = Loc {row: i, col: j},
                _ => ()
            }
        }
    }
    let data = data.iter().map(|l| l.iter().map(|c| { match *c { 
        'S' => 0,
        'E' => 25,
        _ => *c as usize - 'a' as usize
    }}).collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("Part 1: {}", best_path(&start, &end, &data));

    let mut min_dist = i32::MAX;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 0 {
                let path_len = best_path(&Loc {row: i, col: j}, &end, &data);
                min_dist = min(min_dist, path_len);
            }
        }
    }
    println!("Part 2: {}", min_dist);
}
