use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn path_to_string(path: &PathBuf) -> String {
    path.to_str().unwrap().to_string()
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let lines = f.split("\n");

    let mut cur_path = PathBuf::new();
    let mut path_sizes: HashMap<String, u32> = HashMap::new();

    lines.for_each(|line| {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();

        match tokens[0] {
            "$" => match tokens[1] {
                "cd" => {
                    if tokens[2] == ".." {
                        cur_path.pop();
                    } else {
                        cur_path.push(tokens[2]);
                    }
                }
                "ls" => {
                    path_sizes.insert(path_to_string(&cur_path), 0);
                }
                _ => (),
            },
            "dir" => (),
            _ => {
                //assume its a number...
                path_sizes
                    .entry(path_to_string(&cur_path))
                    .and_modify(|e| *e += tokens[0].parse::<u32>().unwrap());
            }
        }
    });

    let mut keys: Vec<String> = path_sizes
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>();

    //from shortest to longest path, scan all longer paths, add each contained path's size
    keys.sort_by(|a, b| {
        if a.len() == b.len() {
            a.cmp(b)
        } else {
            a.len().cmp(&b.len())
        }
    });
    keys.iter().enumerate().for_each(|(i, k1)| {
        keys.iter().skip(i + 1).for_each(|k2| {
            if k2.contains(k1) {
                let child_size = path_sizes[k2];
                path_sizes
                    .entry(k1.to_string())
                    .and_modify(|e| *e += child_size);
            }
        });
    });

    let total_small_space: u32 = path_sizes.values().filter(|v| **v <= 100000).sum();
    println!("Part 1: {}", total_small_space);

    let left_over = 70000000 - path_sizes["/"];
    let needed = if left_over >= 30000000 {
        0
    } else {
        30000000 - left_over
    };

    let nuke_space: u32 = *path_sizes
        .values()
        .filter(|v| **v >= needed)
        .reduce(|acc, value| cmp::min(acc, value))
        .unwrap();

    println!("Part 2: {}", nuke_space);
}
