use std::fs;

const SIZE: usize = 99;

fn row(i: usize) -> usize {
    i / SIZE
}

fn col(i: usize) -> usize {
    i % SIZE
}

fn row_it(row: usize) -> impl DoubleEndedIterator<Item = usize> {
    row * SIZE..row * SIZE + SIZE
}

fn col_it(col: usize) -> impl DoubleEndedIterator<Item = usize> {
    (col..SIZE * SIZE).step_by(SIZE)
}

fn sweep(trees: &Vec<i8>, seen: &mut Vec<bool>, it: impl Iterator<Item = usize>) {
    let mut max: i8 = -1;
    for i in it {
        if trees[i] > max {
            max = trees[i];
            seen[i] = true;
        }
    }
}

fn look(trees: &Vec<i8>, mut it: impl Iterator<Item = usize>) -> u32 {
    let start_height = trees[it.next().unwrap()];
    let mut count = 0;
    for i in it {
        count += 1;
        if trees[i] >= start_height {
            break;
        }
    }
    count
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let trees = f
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c as i8 - '0' as i8)
        .collect::<Vec<i8>>();

    let mut seen: Vec<bool> = vec![false; SIZE * SIZE];
    for i in 0..SIZE {
        sweep(&trees, &mut seen, col_it(col(i)));
        sweep(&trees, &mut seen, row_it(row(i * SIZE)));
        sweep(&trees, &mut seen, col_it(col(i)).rev());
        sweep(&trees, &mut seen, row_it(row(i * SIZE)).rev());
    }
    let vis_count = seen.iter().filter(|v| **v).count();

    println!("Part 1: {:#?}", vis_count);

    let max_scene_score = (0..SIZE * SIZE)
        .map(|i| {
            let down = look(&trees, col_it(col(i)).skip(row(i)));
            let up = look(&trees, col_it(col(i)).rev().skip(SIZE - row(i) - 1));
            let right = look(&trees, row_it(row(i)).skip(col(i)));
            let left = look(&trees, row_it(row(i)).rev().skip(SIZE - col(i) - 1));

            left * right * up * down
        })
        .max();

    println!("Part 2: {:#?}", max_scene_score.unwrap());
}
