use std::cmp::{max, min};
use std::collections::{HashSet, LinkedList, VecDeque};
use std::fs;
use std::mem::swap;

use nom::bytes::complete::take_till;
use nom::character::complete::{i32, i64, line_ending};
use nom::sequence::{preceded, terminated};
use nom::IResult;
use nom::{bytes::complete::tag, combinator::opt, multi::many1};

#[derive(Debug)]
struct EncryptedFile {
    nums: Vec<(usize, i64)>,
}

fn line_parse(input: &str) -> IResult<&str, EncryptedFile> {
    let (i, mut res) = many1(terminated(i64, opt(line_ending)))(input)?;
    let file = EncryptedFile {
        nums: res
            .drain(0..)
            .map(|v| v * 811589153)
            .enumerate()
            .collect::<Vec<_>>(),
    };
    Ok((i, file))
}
fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let mut file = line_parse(&f).unwrap().1;
    println!("{:?}", &file);

    let shifts_per_loop: i64 = file.nums.len() as i64 - 1;

    for _loop in 0..10 {
        for i in 0..file.nums.len() {
            let cur_ind = file.nums.iter().position(|n| n.0 == i).unwrap();
            let cur_val = file.nums[cur_ind];
            let shift_amt = cur_val.1;
            let mut new_index = cur_ind as i64 + shift_amt;
            new_index %= shifts_per_loop;
            if new_index < 0 {
                new_index += shifts_per_loop;
            }
            let new_index = new_index as usize;
            if new_index > cur_ind {
                file.nums[cur_ind..=new_index].rotate_left(1);
            } else {
                file.nums[new_index..=cur_ind].rotate_right(1);
            }
        }
    }
    
    let zero_index = file.nums.iter().position(|n| n.1 == 0).unwrap();
    let ind_1k: usize = (zero_index + 1000) % file.nums.len();
    let ind_2k: usize = (zero_index + 2000) % file.nums.len();
    let ind_3k: usize = (zero_index + 3000) % file.nums.len();
    println!("{}", zero_index);
    println!(
        "{} {} {}",
        file.nums[ind_1k].1, file.nums[ind_2k].1, file.nums[ind_3k].1
    );
    println!(
        "Part 2 : {}",
        file.nums[ind_1k].1 + file.nums[ind_2k].1 + file.nums[ind_3k].1
    );
}
