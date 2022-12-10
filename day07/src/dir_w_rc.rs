use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Dir {
    size: i32,
    //parent: Option<Weak<RefCell<Dir>>>,
    children: BTreeMap<String, Rc<RefCell<Dir>>>,
}

impl Dir {
    // fn new(parent: Option<Weak<RefCell<Dir>>>) -> Dir {
    //     Dir { parent, size: 0, children: BTreeMap::new() }
    // }
    fn new() -> Dir {
        Dir {
            size: 0,
            children: BTreeMap::new(),
        }
    }

    fn size_rec(&self) -> i32 {
        self.size
            + self
                .children
                .iter()
                .map(|c| c.1.borrow().size_rec())
                .sum::<i32>()
    }
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap();
    let lines = f.lines();

    let mut dir_stack: Vec<Rc<RefCell<Dir>>> = Vec::new();
    dir_stack.push(Rc::new(RefCell::new(Dir::new()))); // root

    lines
        .skip(1) //root is special
        .for_each(|line| {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();

            match tokens[0] {
                "$" => match tokens[1] {
                    "cd" => {
                        if tokens[2] == ".." {
                            dir_stack.pop();
                            //cur = cur.upgrade().unwrap().borrow().parent.unwrap();
                        } else {
                            //let cur_dir = cur.upgrade().unwrap().borrow_mut();
                            // let split_loc = dir_stack.len() - 2;
                            // let = dir_stack.split_at_mut(split_loc);
                            let dir_name = tokens[2];
                            let mut dir_exists = false;
                            {
                                let cur_dir = dir_stack.last();
                                match cur_dir {
                                    Some(dir) => {
                                        dir_exists = dir.borrow().children.contains_key(dir_name);
                                    }
                                    _ => {}
                                }
                            }
                            if !dir_exists {
                                //cur_dir.children.insert(tokens[2].to_string(), Rc::new(RefCell::new(Dir::new(cur))));
                                let new_node = Rc::new(RefCell::new(Dir::new()));
                                {
                                    let mut cur_dir_mut = dir_stack.last().unwrap().borrow_mut();
                                    cur_dir_mut
                                        .children
                                        .insert(tokens[2].to_string(), new_node.clone());
                                }
                                dir_stack.push(new_node);
                            }
                        }
                    }
                    "ls" => {
                        let mut cur_dir = dir_stack.last().unwrap().borrow_mut();
                        cur_dir.size = 0;
                    }
                    _ => (),
                },
                "dir" => (),
                _ => {
                    //assume its a number...
                    let mut cur_dir = dir_stack.last().unwrap().borrow_mut();
                    cur_dir.size += tokens[0].parse::<i32>().unwrap();
                }
            }
        });


    let root = dir_stack[0].borrow();
    dbg!(&root);
}
