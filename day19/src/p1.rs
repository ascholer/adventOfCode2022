use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::fs;

use nom::bytes::complete::take_till;
use nom::character::complete::i32;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use nom::{bytes::complete::tag, combinator::opt, multi::many1};

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug)]
struct Blueprint {
    costs: Vec<Vec<i32>>,
    max_ore: i32,
}

fn line_parse(input: &str) -> IResult<&str, Blueprint> {
    let (i, res) = many1(preceded(take_till(|c: char| c.is_numeric()), i32))(input)?;
    let costs = vec![
        vec![res[1]],
        vec![res[2]],
        vec![res[3], res[4]],
        vec![res[5], 0, res[6]],
    ];
    let max_ore = costs.iter().map(|c| c[0]).max().unwrap();
    let bluep = Blueprint { costs, max_ore };

    Ok((i, bluep))
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    resources: [i32; 4],
    bots: [i32; 4],
    time: i32,
}

fn gen_states(s: &State, bp: &Blueprint) -> Vec<State> {
    let mut states: Vec<State> = Vec::new();
    let mut no_bot = (*s).clone();
    no_bot.time -= 1;
    for ri in 0..4 {
        no_bot.resources[ri] += no_bot.bots[ri];
    }

    let mut need_no = true;

    if s.resources[ORE] >= bp.costs[GEODE][ORE]
        && s.resources[OBSIDIAN] >= bp.costs[GEODE][OBSIDIAN]
    {
        let mut bot = no_bot.clone();
        bot.bots[GEODE] += 1;
        bot.resources[ORE] -= bp.costs[GEODE][ORE];
        bot.resources[OBSIDIAN] -= bp.costs[GEODE][OBSIDIAN];
        states.push(bot);
        need_no = false;
    }

    if s.time >= 4
        && s.resources[ORE] >= bp.costs[OBSIDIAN][ORE]
        && s.resources[CLAY] >= bp.costs[OBSIDIAN][CLAY]
    {
        let mut bot = no_bot.clone();
        bot.bots[OBSIDIAN] += 1;
        bot.resources[ORE] -= bp.costs[OBSIDIAN][ORE];
        bot.resources[CLAY] -= bp.costs[OBSIDIAN][CLAY];
        states.push(bot);
    }

    if s.time >= 6
        && s.bots[CLAY] < bp.costs[OBSIDIAN][CLAY]
        && s.resources[ORE] >= bp.costs[CLAY][ORE]
    {
        let mut bot = no_bot.clone();
        bot.bots[CLAY] += 1;
        bot.resources[ORE] -= bp.costs[CLAY][ORE];
        states.push(bot);
    }

    if s.time >= 4 && s.bots[ORE] < bp.max_ore && s.resources[ORE] >= bp.costs[ORE][ORE]
    {
        let mut bot = no_bot.clone();
        bot.bots[ORE] += 1;
        bot.resources[ORE] -= bp.costs[ORE][ORE];
        states.push(bot);
    }

    if need_no {
        states.push(no_bot);
    }

    states
}

fn max_fut_geodes(s: &State) -> i32 {
    let base = s.resources[GEODE];
    let cur_prod = s.bots[GEODE] * s.time;
    let future_prod = (s.time * s.time - s.time) / 2; //(n^2 - n)/2
    base + cur_prod + future_prod
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let f = fs::read_to_string("data.txt").unwrap();
    let bprints: Vec<Blueprint> = f.lines().map(|l| line_parse(l).unwrap().1).collect();
    let mut index = 1;
    let mut total_quality = 0;
    for bp in bprints {
        println!("{:?}", &bp);
        let mut opts: VecDeque<State> = VecDeque::new();
        opts.push_back(State {
            resources: [0; 4],
            bots: [1, 0, 0, 0],
            time: 24,
        });
        let mut seen: HashSet<State> = HashSet::new();

        let mut max_geodes = 0;
        let mut states_considerd: u64 = 0;
        while opts.len() > 0 {
            let s = opts.pop_front().unwrap();
            states_considerd += 1;
            if s.time < 5 && max_fut_geodes(&s) < max_geodes {
                //Discard
            } else {
                if s.time == 0 {
                    max_geodes = max(max_geodes, s.resources[GEODE]);
                } else {
                    let mut new_states = gen_states(&s, &bp);
                    new_states.iter().rev().for_each(|s| {
                        if !seen.contains(s) {
                            seen.insert(*s);
                            opts.push_front(*s);
                        }
                    });
                }
            }
        }
        let quality = max_geodes * index;
        total_quality += quality;
        println!("{:?} {:?} {:?}", index, max_geodes, quality);
        println!("{:?}", states_considerd);
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        index += 1;
    }
    println!("{:?}", total_quality);
}
