use std::hash::Hash;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    fs,
};

type Loc = (i32, i32);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Gust {
    loc: Loc,
    dir: char,
}
impl Ord for Gust {
    fn cmp(&self, other: &Self) -> Ordering {
        self.loc
            .0
            .cmp(&other.loc.0)
            .then_with(|| self.loc.1.cmp(&other.loc.1))
    }
}

impl PartialOrd for Gust {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct WindState {
    gusts: Vec<Gust>,
    blocked: HashSet<Loc>,
}

#[derive(Debug, Clone, Eq)]
struct ExpeditionState {
    expedition: Loc,
    elapsed: i32,
    est_cost: i32,
    cur_goal: Loc,
}

impl Display for ExpeditionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} : {:?} : {:?}",
            self.expedition, self.elapsed, self.est_cost
        )
    }
}

impl PartialEq for ExpeditionState {
    fn eq(&self, other: &Self) -> bool {
        self.est_cost == other.est_cost
    }
}

impl Ord for ExpeditionState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.est_cost.cmp(&self.est_cost)
    }
}

impl PartialOrd for ExpeditionState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ExpeditionState {
    fn new(expedition: Loc, elapsed: i32, cur_goal: Loc) -> ExpeditionState {
        let est_cost =
            elapsed + (cur_goal.0 - expedition.0).abs() + (cur_goal.1 - expedition.1).abs();
        ExpeditionState {
            expedition,
            elapsed,
            est_cost,
            cur_goal,
        }
    }
}

#[derive(Eq)]
struct StateHash(ExpeditionState);

impl Hash for StateHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expedition.hash(state);
        self.0.elapsed.hash(state);
    }
}

impl PartialEq for StateHash {
    fn eq(&self, other: &Self) -> bool {
        self.0.expedition == other.0.expedition && self.0.elapsed == other.0.elapsed
    }
}

const WIDTH: i32 = 122;
const HEIGHT: i32 = 27;

const START_LOC: Loc = (0, 1);
const END_LOC: Loc = (HEIGHT - 1, WIDTH - 2);

fn get_moves(loc: &Loc, blocked: &HashSet<Loc>) -> Vec<Loc> {
    let mut moves = Vec::new();
    for m in [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)] {
        let new_loc = (loc.0 + m.0, loc.1 + m.1);
        if (new_loc.0 > 0 || new_loc.1 == 1)
            && (new_loc.0 < HEIGHT - 1 || new_loc.1 == WIDTH - 2)
            && new_loc.1 > 0
            && new_loc.1 < WIDTH - 1
            && !blocked.contains(&new_loc)
        {
            moves.push(new_loc);
        }
    }
    moves
}

fn wind_step(old_gusts: &Vec<Gust>) -> Vec<Gust> {
    const WIDTH_MOD: i32 = WIDTH - 2;
    const HEIGHT_MOD: i32 = HEIGHT - 2;
    let mut gusts: Vec<Gust> = Vec::new();
    for o_g in old_gusts {
        let loc = match o_g.dir {
            '>' => {
                let new_col = (o_g.loc.1 % WIDTH_MOD) + 1;
                (o_g.loc.0, new_col)
            }
            '<' => {
                let new_col = ((o_g.loc.1 - 2 + WIDTH_MOD) % WIDTH_MOD) + 1;
                (o_g.loc.0, new_col)
            }
            'v' => {
                let new_row = (o_g.loc.0 % HEIGHT_MOD) + 1;
                (new_row, o_g.loc.1)
            }
            '^' => {
                let new_row = ((o_g.loc.0 - 2 + HEIGHT_MOD) % HEIGHT_MOD) + 1;
                (new_row, o_g.loc.1)
            }
            _ => {
                panic!("WTH")
            }
        };
        gusts.push(Gust { loc, dir: o_g.dir });
    }
    gusts
}

fn get_or_gen_wind(time: i32, wind_states: &mut HashMap<i32, WindState>) -> &WindState {
    if !wind_states.contains_key(&time) {
        let new_gusts = wind_step(&wind_states[&(time - 1)].gusts);
        let new_blocked = new_gusts.iter().map(|a| a.loc).collect::<HashSet<Loc>>();
        let wind = WindState {
            gusts: new_gusts,
            blocked: new_blocked,
        };
        wind_states.insert(time, wind);
    }
    wind_states.get(&time).unwrap()
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let f = fs::read_to_string("data.txt").unwrap();
    let mut gusts: Vec<Gust> = Vec::new();
    for (i, l) in f.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {}
                'E' => {}
                _ => {
                    gusts.push(Gust {
                        loc: (i as i32, j as i32),
                        dir: c,
                    });
                }
            }
        }
    }
    let blocked: HashSet<Loc> = gusts.iter().map(|a| a.loc).collect::<HashSet<Loc>>();

    let mut wind_states: HashMap<i32, WindState> = HashMap::new();
    wind_states.insert(0, WindState { gusts, blocked });

    let initial_state = ExpeditionState::new(START_LOC, 0, END_LOC.clone());
    let mut seen: HashSet<StateHash> = HashSet::new();
    let mut states: BinaryHeap<ExpeditionState> = BinaryHeap::new();
    seen.insert(StateHash(initial_state.clone()));
    states.push(initial_state);

    let mut stage = 1; //Start of search
    while states.len() > 0 {
        let mut s = states.pop().unwrap();

        if s.expedition == s.cur_goal {
            println!("Done with stage {} in {}", stage, s.elapsed);
            let new_goal = match stage {
                1 => START_LOC,
                2 => END_LOC,
                _ => {
                    let elapsed = now.elapsed();
                    println!("Elapsed: {:.2?}", elapsed);
                    break;
                }
            };

            stage += 1;
            seen.clear();
            states.clear();

            s.cur_goal = new_goal;
            seen.insert(StateHash(s.clone()));
        }

        let next_wind = get_or_gen_wind(s.elapsed + 1, &mut wind_states);

        let options = get_moves(&s.expedition, &next_wind.blocked);

        for o in options {
            let sn = ExpeditionState::new(o, s.elapsed + 1, s.cur_goal);
            let snh = StateHash(sn.clone());
            if !seen.contains(&snh) {
                seen.insert(StateHash(sn.clone()));
                states.push(sn);
            }
        }
    }
}
