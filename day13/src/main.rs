use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    multi::many0,
    sequence::{delimited, terminated},
    IResult,
};
use std::{any::Any, cmp::Ordering, fs};

trait ListItem {
    fn is_list(&self) -> bool; //todo - figure out downcast
    fn to_string(&self) -> String;
    fn compare(&self, o: &dyn ListItem) -> Ordering;
    fn as_any(&self) -> &dyn Any;
}

impl ListItem for i32 {
    fn is_list(&self) -> bool {
        false
    }

    fn to_string(&self) -> String {
        ToString::to_string(&self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn compare(&self, o: &dyn ListItem) -> Ordering {
        if o.is_list() {
            let mut self_list = List { items: Vec::new() };
            self_list
                .items
                .push(Box::<dyn ListItem>::from(Box::new(*self)));
            self_list.compare(o)
        } else {
            let o = o.as_any().downcast_ref::<i32>().unwrap();
            self.cmp(o)
        }
    }
}

struct List {
    items: Vec<Box<dyn ListItem>>,
}

impl ListItem for List {
    fn is_list(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_string(&self) -> String {
        let contents: String = self
            .items
            .iter()
            .map(|i| {
                let mut s = i.as_ref().to_string();
                s.push(',');
                s
            })
            .collect();
        let mut s = "[".to_string();
        s.push_str(&contents);
        s.push(']');
        s
    }

    fn compare(&self, o: &dyn ListItem) -> Ordering {
        if o.is_list() {
            let o = o.as_any().downcast_ref::<List>().unwrap();
            for i in 0..self.items.len() {
                if i >= o.items.len() {
                    return Ordering::Greater; //exhaust right
                } else {
                    let res = self.items[i].compare(&*o.items[i]);
                    match res {
                        Ordering::Equal => {
                            continue;
                        }
                        _ => {
                            return res;
                        }
                    }
                }
            }
            //got to end
            self.items.len().cmp(&o.items.len())
        } else {
            let o = o.as_any().downcast_ref::<i32>().unwrap();
            let mut other_list = List { items: Vec::new() };
            other_list
                .items
                .push(Box::<dyn ListItem>::from(Box::new(*o)));
            self.compare(&other_list)
        }
    }
}

fn num_parse(input: &str) -> IResult<&str, Box<dyn ListItem>> {
    let (i, res) = digit1(input)?;
    let item = Box::<dyn ListItem>::from(Box::new(res.parse::<i32>().unwrap()));
    Ok((i, item))
}

fn list_parse(input: &str) -> IResult<&str, Box<dyn ListItem>> {
    let (i, res) = delimited(
        tag("["),
        many0(terminated(alt((num_parse, list_parse)), opt(tag(",")))),
        tag("]"),
    )(input)?;
    let list = Box::<dyn ListItem>::from(Box::new(List { items: res }));
    Ok((i, list))
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap() + &"\n\n[[2]]\n[[6]]";
    let pairs = f.split("\n\n");

    let mut packets = pairs
        .flat_map(|p| {
            p.lines()
                .map(|l| list_parse(l).unwrap().1)
                .collect::<Vec<Box<dyn ListItem>>>()
        })
        .collect::<Vec<Box<dyn ListItem>>>();

    packets.sort_by(|a, b| a.compare(&**b));

    let pack2 = list_parse("[[2]]").unwrap().1;
    let pack6 = list_parse("[[6]]").unwrap().1;

    let total = packets
        .iter()
        .enumerate()
        .filter(|p| {
            p.1.compare(&*pack2) == Ordering::Equal || p.1.compare(&*pack6) == Ordering::Equal
        })
        .map(|p| p.0 + 1)
        .fold(1, |acc, i| acc * i);

    println!("Part 2: {:?}", total);
}
