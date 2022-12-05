use std::fs;
use std::iter::zip;

#[derive(Debug)]
struct Command {
    number : usize,
    from : usize,
    to: usize,
}

fn p1(commands : &Vec<Command>, mut stacks : Vec<Vec<char>> ) -> String {
    for c in commands{
        for _i in 0..c.number {
            let val = stacks[c.from].pop().unwrap();
            stacks[c.to].push(val);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}

fn p2(commands : &Vec<Command>, mut stacks : Vec<Vec<char>> ) -> String {
    for c in commands{
        let split_index = stacks[c.from].len() - c.number;
        let values = stacks[c.from].drain(split_index..).collect::<Vec<_>>();
        values.iter().for_each(|v| stacks[c.to].push(*v));
    }
    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let lines = f.split("\n").collect::<Vec<&str>>();
    let num_stacks = (lines[0].chars().count() + 1) / 4;
    let stack_base_row = lines.iter().position(|l| l.chars().all(|c| c != '[')).unwrap();

    let mut stacks : Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    for line in lines.iter().take(stack_base_row).rev() {
        let insertions = zip(0..num_stacks, line.chars().skip(1).step_by(4));
        insertions.for_each(|i| if i.1 != ' ' { stacks[i.0].push(i.1) });
    }

    let command_lines = lines.iter().skip(stack_base_row + 2);
    let commands = command_lines.map(|l| l.split_whitespace().skip(1).step_by(2).map(|p| p.parse::<usize>().unwrap()));
    let commands = commands.map(|mut c| Command { number: c.next().unwrap(), from: c.next().unwrap() - 1, to: c.next().unwrap() - 1}).collect::<Vec<Command>>();
    
    let stacks_copy = stacks.clone();
    println!("Part 1: {:#?}", p1(&commands, stacks_copy));
    println!("Part 2: {:#?}", p2(&commands, stacks));
}
