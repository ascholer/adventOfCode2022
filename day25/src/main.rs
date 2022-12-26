use std::fs;

fn read_snafu(line: &str) -> i64 {
    let mut cur_value = 1;
    let mut sum = 0;
    for c in line.chars().rev() {
        let val = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("yikes"),
        };
        sum += val * cur_value;
        cur_value *= 5;
    }
    sum
}

fn make_snafu(mut val: i64) -> String {
    let mut snafu = String::new();
    while val > 0 {
        let part = val % 5;
        let digit = match part {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                val += 2;
                '='
            }
            _ => {
                val += 1;
                '-'
            }
        };
        snafu.insert(0, digit);
        val /= 5;
    }
    snafu
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let mut sum = 0;
    for l in f.lines() {
        sum += read_snafu(l);
    }
    println!("{}", sum);
    println!("{}", make_snafu(sum));
}
