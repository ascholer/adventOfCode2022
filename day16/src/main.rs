use std::collections::{BTreeSet, HashMap, VecDeque, BinaryHeap};
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

#[derive(Eq, Clone, Debug)]
struct Path {
    path: Vec<String>,
    el_path: Vec<String>,
    opened: BTreeSet<String>,
    released: i32,
    time_left: i32,
    open_rec: Vec<String>,
    can_backtrack: bool,
    el_can_backtrack : bool
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.el_path == other.el_path && self.opened == other.opened && self.released == other.released && self.time_left == other.time_left && self.open_rec == other.open_rec && self.can_backtrack == other.can_backtrack && self.el_can_backtrack == other.el_can_backtrack
    }
}


#[derive(PartialEq, Eq, Debug)]
struct PathCmp(Path);

impl PartialOrd for PathCmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathCmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.released.cmp(&other.0.released)
    }
}


fn future_potential(p: &Path, working_valves: &BTreeSet<ValveCmp>) -> i32 {
    let mut potential = 0;
    let mut t_left = p.time_left - 1;
    let mut open_count = 0;
    for v in working_valves {
        if !p.opened.contains(&v.0.name) {
            potential = potential + &v.0.flow * t_left;
            open_count += 1;
            if open_count % 2 == 0 {
                t_left -= 2;
                if t_left <= 0 {
                    break;
                }
            }
        }
    }
    potential
}

fn gen_options(
    path: &Path,
    valves: &HashMap<String, Valve>,
    just_opened: bool,
    el_just_opened: bool
) -> Vec<PathCmp>{

    let mut options : Vec<PathCmp> = Vec::new();
    if just_opened && !el_just_opened {
        //just move el
        let el_tunnels = &valves.get(path.el_path.last().unwrap()).unwrap().tunnels;
        for el_t in el_tunnels {
            if !path.el_can_backtrack && path.el_path.len() > 1 && &path.el_path[path.el_path.len() - 2] == el_t {
                continue;
            }
            let mut new_path = path.clone();
            new_path.el_can_backtrack = false;
            new_path.el_path.push(el_t.to_string());
            options.push(PathCmp(new_path));
        }
    }
    if !just_opened && el_just_opened {
        //just move self
        let tunnels = &valves.get(path.path.last().unwrap()).unwrap().tunnels;
        for t in tunnels {
            if !path.can_backtrack && path.path.len() > 1 && &path.path[path.path.len() - 2] == t {
                continue;
            }
            let mut new_path = path.clone();
            new_path.path.push(t.to_string());
            new_path.can_backtrack = false;
            options.push(PathCmp(new_path));
        }
    }
    if just_opened && el_just_opened {
        //no moves, just record
        let mut new_path = path.clone();
        options.push(PathCmp(new_path));
    }
    if !just_opened && !el_just_opened {
        //both move
        let tunnels = &valves.get(path.path.last().unwrap()).unwrap().tunnels;
        let el_tunnels = &valves.get(path.el_path.last().unwrap()).unwrap().tunnels;
        for t in tunnels {
            if !path.can_backtrack && path.path.len() > 1 && &path.path[path.path.len() - 2] == t {
                continue;
            }
            for el_t in el_tunnels {
                if ( !path.el_can_backtrack && path.el_path.len() > 1 && &path.el_path[path.el_path.len() - 2] == el_t) ||
                (t == el_t && path.path.len() >= 1 && path.el_path.len() >= 1 && &path.path[path.path.len() - 1] == &path.el_path[path.el_path.len() - 1]) {
                    let x = 1;
                    continue;
                }
                let mut new_path = path.clone();
                new_path.path.push(t.to_string());
                new_path.el_path.push(el_t.to_string());
                new_path.can_backtrack = false;
                new_path.el_can_backtrack = false;
                options.push(PathCmp(new_path));
            }
        }

    }
    options
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let f = fs::read_to_string("data.txt").unwrap();

    let mut valves: HashMap<String, Valve> = HashMap::new();
    let mut options: BinaryHeap<PathCmp> = BinaryHeap::new();
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
        el_path: vec!["AA".to_string(); 1],
        opened: BTreeSet::new(),
        released: 0,
        time_left: 26,
        open_rec: Vec::new(),
        can_backtrack: false,
        el_can_backtrack: false
    };
    options.push(PathCmp(start));

    let mut best_release = i32::MIN;

    let mut count = 0;

    while options.len() > 0 {
        let cur_path = options.pop().unwrap().0;
        count += 1;
        if count % 100000 == 0 {
            println!("{:?}", &options.len());
            println!("{:?}", &best_release);
            println!("{:?}", &cur_path);
            count = 0;
        }
        
        if cur_path.open_rec.len() >= 2 && &cur_path.open_rec[0] == "DD 24" && &cur_path.open_rec[1] == "JJ 23" {
            let x = 1;
        }

        if cur_path.time_left == 0 || cur_path.opened.len() == num_to_open {
            completed.push(cur_path);
        } else {
            //See if we can prune option
            let best_case = cur_path.released + future_potential(&cur_path, &working_valves);
            if best_case < best_release {
                if cur_path.open_rec.len() >= 3 && &cur_path.open_rec[0] == "DD 24" && &cur_path.open_rec[1] == "JJ 23" {
                    let x = 1;
                }
                continue;
            }

            //Check if can open valve, if so it is a new option - open valve and generate next steps
            let cur_loc = cur_path.path.last().unwrap();
            let cur_valve = &valves[cur_loc];
            let cur_el_loc = cur_path.el_path.last().unwrap();
            let cur_el_valve = &valves[cur_el_loc];

            let i_can_open = cur_valve.flow > 0 && !cur_path.opened.contains(&cur_valve.name);
            let el_can_open = cur_el_valve.flow > 0 && !cur_path.opened.contains(&cur_el_valve.name);

            if i_can_open {
                if el_can_open && cur_loc != cur_el_loc {
                    //everyone opens
                    let mut path = cur_path.clone();
                    path.time_left -= 1;
                    path.released += path.time_left * cur_valve.flow;
                    path.opened.insert(cur_valve.name.clone());
                    path.can_backtrack = true;
                    path.open_rec
                        .push(cur_valve.name.clone() + " " + &path.time_left.to_string().to_owned());
                    path.released += path.time_left * cur_el_valve.flow;
                    path.opened.insert(cur_el_valve.name.clone());
                    path.el_can_backtrack = true;
                    path.open_rec
                        .push(cur_el_valve.name.clone() + " " + &path.time_left.to_string().to_owned());
                    if path.released > best_release {
                        best_release = path.released;
                    }
                    let mut opts = gen_options(&path, &valves, true, true);
                    //println!("New: {:?}", &opts);
                    opts.drain(0..).for_each(|o| {
                        options.push(o);
                    });
                } 
                //I open
                let mut path = cur_path.clone();
                path.time_left -= 1;
                path.released += path.time_left * cur_valve.flow;
                path.opened.insert(cur_valve.name.clone());
                path.can_backtrack = true;
                path.open_rec
                    .push(cur_valve.name.clone() + " " + &path.time_left.to_string().to_owned());
                if path.released > best_release {
                    best_release = path.released;
                }
                let mut opts = gen_options(&path, &valves, true, false);
                //println!("New: {:?}", &opts);
                opts.drain(0..).for_each(|o| {
                    options.push(o);
                });
            }
            if el_can_open {
                //el open
                let mut path = cur_path.clone();
                path.time_left -= 1;
                path.released += path.time_left * cur_el_valve.flow;
                path.opened.insert(cur_el_valve.name.clone());
                path.el_can_backtrack = true;
                path.open_rec
                    .push(cur_el_valve.name.clone() + " " + &path.time_left.to_string().to_owned());
                if path.released > best_release {
                    best_release = path.released;
                }
                let mut opts = gen_options(&path, &valves, false, true);
                //println!("New: {:?}", &opts);
                opts.drain(0..).for_each(|o| {
                    options.push(o);
                });
            }

            //Now check all tunnels we could go into from here without an open
            let mut path = cur_path.clone();
            path.time_left -= 1;
            let mut opts = gen_options(&path, &valves, false, false);
            //println!("New: {:?}", &opts);
            opts.drain(0..).for_each(|o| {
                options.push(o);
            });
        }
        
        // println!("{:?}", &options.peek());
        // println!("-------------------------------------------");
        // for o in &options {
        //     println!("{:?}", &o);
        // }
        // println!("-------------------------------------------");
        // println!("-------------------------------------------");
    }

    completed.sort_by(|a, b| b.released.cmp(&a.released));

    dbg!(&completed[0]);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
