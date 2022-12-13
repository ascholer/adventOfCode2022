use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::opt,
    multi::many0,
    sequence::{delimited, terminated},
    IResult,
};
use std::{cmp::Ordering, fs};

enum ListItem {
    Num(i32),
    List(List),
}
struct List {
    items: Vec<ListItem>,
}

fn compare(a: &ListItem, b: &ListItem) -> Ordering {
    match &a {
        ListItem::Num(number_a) => match &b {
            ListItem::Num(number_b) => number_a.cmp(&number_b),
            ListItem::List(_list_b) => {
                let list_a = ListItem::List(List {
                    items: vec![ListItem::Num(*number_a)],
                });
                compare(&list_a, b)
            }
        },
        ListItem::List(list_a) => {
            match &b {
                ListItem::Num(number_b) => {
                    let list_b = ListItem::List(List {
                        items: vec![ListItem::Num(*number_b)],
                    });
                    compare(a, &list_b)
                }
                ListItem::List(list_b) => {
                    for i in 0..list_a.items.len() {
                        if i >= list_b.items.len() {
                            return Ordering::Greater; //exhaust right
                        } else {
                            let res = compare(&list_a.items[i], &list_b.items[i]);
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
                    list_a.items.len().cmp(&list_b.items.len())
                }
            }
        }
    }
}

fn num_parse(input: &str) -> IResult<&str, ListItem> {
    let (i, res) = digit1(input)?;
    let item = ListItem::Num(res.parse::<i32>().unwrap());
    Ok((i, item))
}

fn list_parse(input: &str) -> IResult<&str, ListItem> {
    let (i, res) = delimited(
        tag("["),
        many0(terminated(alt((num_parse, list_parse)), opt(tag(",")))),
        tag("]"),
    )(input)?;
    let list = ListItem::List(List { items: res });
    Ok((i, list))
}

fn main() {
    let f = fs::read_to_string("data.txt").unwrap() + &"\n\n[[2]]\n[[6]]";
    let pairs = f.split("\n\n");

    let mut packets = pairs
        .flat_map(|p| {
            p.lines()
                .map(|l| list_parse(l).unwrap().1)
                .collect::<Vec<ListItem>>()
        })
        .collect::<Vec<ListItem>>();

    packets.sort_by(|a, b| compare(&a, &b));

    let pack2 = list_parse("[[2]]").unwrap().1;
    let pack6 = list_parse("[[6]]").unwrap().1;

    let total = packets
        .iter()
        .enumerate()
        .filter(|p| {
            compare(&p.1, &pack2) == Ordering::Equal || compare(&p.1, &pack6) == Ordering::Equal
        })
        .map(|p| p.0 + 1)
        .fold(1, |acc, i| acc * i);

    println!("Part 2: {:?}", total);
}
