use std::collections::{BTreeSet, HashMap, VecDeque};
use std::fs;

use nom::branch::alt;
use nom::character::complete::{alpha1, i32};
use nom::sequence::{pair, preceded, terminated};
use nom::{
    bytes::complete::tag, character::complete::line_ending, combinator::opt, multi::many1,
    sequence::separated_pair, IResult,
};

#[derive(Eq, Clone, Debug)]
struct Valve {
    name: String,
    flow: i32,
    tunnels: Vec<String>,
}

impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(PartialEq, Eq, Debug)]
struct ValveCmp(Valve);

impl PartialOrd for ValveCmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ValveCmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.flow.cmp(&self.0.flow)
    }
}

fn nom_valve(input: &str) -> IResult<&str, (String, i32)> {
    let (i, res) = preceded(
        opt(tag("Valve ")),
        separated_pair(alpha1, tag(" has flow rate="), i32),
    )(input)?;
    Ok((i, (res.0.to_string(), res.1)))
}

fn nom_tunnel_list(input: &str) -> IResult<&str, Vec<String>> {
    let (i, res) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        many1(terminated(alpha1, opt(tag(", ")))),
    )(input)?;

    let res: Vec<String> = res.iter().map(|s| s.to_string()).collect();

    Ok((i, res))
}

fn line_parse(input: &str) -> IResult<&str, Valve> {
    let (i, res) = terminated(pair(nom_valve, nom_tunnel_list), opt(line_ending))(input)?;

    let res = Valve {
        name: res.0 .0,
        flow: res.0 .1,
        tunnels: res.1,
    };

    Ok((i, res))
}

#[derive(Clone, Debug)]
struct Path {
    path: Vec<String>,
    opened: BTreeSet<String>,
    released: i32,
    time_left: i32,
    open_rec: Vec<String>,
}

fn future_potential(p: &Path, working_valves: &BTreeSet<ValveCmp>) -> i32 {
    let mut potential = 0;
    let mut t_left = p.time_left - 1;
    for v in working_valves {
        if !p.opened.contains(&v.0.name) {
            potential = potential + &v.0.flow * t_left;
            t_left -= 2;
            if t_left <= 0 {
                break;
            }
        }
    }
    potential
}

fn gen_options(
    path: &Path,
    valves: &HashMap<String, Valve>,
    options: &mut VecDeque<Path>,
    allow_backtrack: bool,
) {
    let tunnels = &valves.get(path.path.last().unwrap()).unwrap().tunnels;
    for t in tunnels {
        if !allow_backtrack && path.path.len() > 1 && &path.path[path.path.len() - 2] == t {
            continue;
        }
        let mut new_path = path.clone();
        new_path.path.push(t.to_string());
        new_path.time_left -= 1;
        options.push_back(new_path);
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let f = fs::read_to_string("data.txt").unwrap();

    let mut valves: HashMap<String, Valve> = HashMap::new();
    let mut options: VecDeque<Path> = VecDeque::new();
    let mut completed: Vec<Path> = Vec::new();

    for l in f.lines() {
        let v = line_parse(l).unwrap().1;
        valves.insert(v.name.clone(), v);
    }

    let working_valves: BTreeSet<ValveCmp> = valves
        .iter()
        .filter(|kv| kv.1.flow > 0)
        .map(|kv| ValveCmp { 0: kv.1.clone() })
        .collect();

    let num_to_open = valves.values().filter(|v| v.flow > 0).count();

    let start = Path {
        path: vec!["AA".to_string(); 1],
        opened: BTreeSet::new(),
        released: 0,
        time_left: 30,
        open_rec: Vec::new(),
    };
    options.push_back(start);

    let mut best_release = i32::MIN;

    while options.len() > 0 {
        let cur_path = options.pop_front().unwrap();

        if cur_path.time_left == 0 || cur_path.opened.len() == num_to_open {
            completed.push(cur_path);
        } else {
            //See if we can prune option
            let best_case = cur_path.released + future_potential(&cur_path, &working_valves);
            if best_case < best_release {
                continue;
            }

            //Check if can open valve, if so it is a new option - open valve and generate next steps
            let cur_loc = &cur_path.path[cur_path.path.len() - 1];
            let cur_valve = &valves[cur_loc];
            if cur_valve.flow > 0 && !cur_path.opened.contains(&cur_valve.name) {
                let mut path = cur_path.clone();
                path.time_left -= 1;
                path.released += path.time_left * cur_valve.flow;
                path.opened.insert(cur_valve.name.clone());
                path.open_rec
                    .push(cur_valve.name.clone() + " " + &path.time_left.to_string().to_owned());
                if path.released > best_release {
                    best_release = path.released;
                }
                gen_options(&path, &valves, &mut options, true);
            }

            //Now check all tunnels we could go into from here without an open
            gen_options(&cur_path, &valves, &mut options, false);
        }
    }

    completed.sort_by(|a, b| b.released.cmp(&a.released));

    dbg!(&completed[0]);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
