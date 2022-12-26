use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs,
};

const DIRECTIONS: [char; 4] = ['N', 'S', 'W', 'E'];

fn get_directions(offset: usize) {
    let dirs = DIRECTIONS[0..4].rotate_left(1).clone();
    dirs
}

type Loc = (i32, i32);

fn bounds(elves: &HashSet<Loc>) -> (Loc, Loc) {
    let mut rmin = i32::MAX;
    let mut rmax = i32::MIN;
    let mut cmin = i32::MAX;
    let mut cmax = i32::MIN;
    for e in elves {
        rmax = max(rmax, e.0);
        rmin = min(rmin, e.0);
        cmax = max(cmax, e.1);
        cmin = min(cmin, e.1);
    }
    ((rmin, cmin), (rmax, cmax))
}

fn print(elves: &HashSet<Loc>) {
    let (start, end) = bounds(elves);
    for r in start.0..=end.0 {
        for c in start.1..=end.1 {
            if elves.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn get_adj(loc: &Loc, dir: char) -> Vec<Loc> {
    match dir {
        'N' => {
            vec![
                (loc.0 - 1, loc.1 - 1),
                (loc.0 - 1, loc.1),
                (loc.0 - 1, loc.1 + 1),
            ]
        }
        'S' => {
            vec![
                (loc.0 + 1, loc.1 - 1),
                (loc.0 + 1, loc.1),
                (loc.0 + 1, loc.1 + 1),
            ]
        }
        'E' => {
            vec![
                (loc.0 - 1, loc.1 + 1),
                (loc.0, loc.1 + 1),
                (loc.0 + 1, loc.1 + 1),
            ]
        }
        _ => {
            vec![
                (loc.0 - 1, loc.1 - 1),
                (loc.0, loc.1 - 1),
                (loc.0 + 1, loc.1 - 1),
            ]
        }
    }
}

fn are_adj(loc1: &Loc, loc2: &Loc) -> bool {
    if loc1 == loc2 {
        return false;
    }
    (loc1.0 - loc2.0).abs() <= 1 && (loc1.1 - loc2.1).abs() <= 1
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let mut elves: HashSet<Loc> = HashSet::new();
    for (i, l) in f.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    let mut start_dir = 0;
    let mut dirList = ['N', 'S', 'W', 'E'];
    for step in 1.. {
        let mut done = true;
        let mut proposal_from_to: HashMap<Loc, Loc> = HashMap::new();
        let mut proposal_to_count: HashMap<Loc, i32> = HashMap::new();

        for e in &elves {
            if elves.iter().all(|e2| !are_adj(e, e2)) {
                continue;
            }
            for dir in dirList {
                let adj = get_adj(&e, dir);
                if adj.iter().all(|l| !elves.contains(l)) {
                    proposal_from_to.insert(e.clone(), adj[1]);
                    proposal_to_count
                        .entry(adj[1])
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                    done = false;
                    break;
                }
            }
        }
        let proposal_set: HashSet<Loc> = proposal_from_to.keys().map(|k| k.clone()).collect();

        for (k, v) in &proposal_from_to {
            if proposal_to_count[&v] == 1 {
                elves.remove(&k);
                elves.insert(*v);
            }
        }
        dirList[0..4].rotate_left(1);
        if done {
            println!("Part2: {}", step);
            break;
        } else {
            println!("{} : {}", step, proposal_from_to.len())
        }
    }
}
