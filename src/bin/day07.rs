extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum Oddity<T> {
    AllSame,  // aaaa
    OneUnlikeOthers {  // aaaba
        typical: T,
        unique: T,
    },
    TwoDifferent,  // ab
    ManyOfTwoTypes,  // aaabbb
    MoreThanTwoTypes,  // abc
}

// inspired by
// https://nedbatchelder.com/blog/201712/itertools_for_puzzles_oddity.html
fn oddity_by<T, I, F>(xs: I, same: F) -> Oddity<T>
where
    I: Iterator<Item=T>,
    F: Fn(&T, &T) -> bool,
{
    let mut seen: [(Option<T>, i32); 2] = [(None, 0), (None, 0)];
    for x in xs {
        let mut done = false;
        for s in &mut seen {
            match s {
                &mut (Some(ref e), ref mut cnt) => {
                    if same(e, &x) {
                        *cnt += 1;
                        done = true;
                        break;
                    }
                }
                &mut ref mut t @ (None, _) => {
                    *t = (Some(x), 1);
                    done = true;
                    break;
                }
            }
        }
        if !done {
            return Oddity::MoreThanTwoTypes;
        }
    }
    match (seen[0].1, seen[1].1) {
        (_, 0) => Oddity::AllSame,
        (1, 1) => Oddity::TwoDifferent,
        (_, 1) => Oddity::OneUnlikeOthers {
            typical: std::mem::replace(&mut seen[0].0, None).unwrap(),
            unique: std::mem::replace(&mut seen[1].0, None).unwrap(),
        },
        (1, _) => Oddity::OneUnlikeOthers {
            typical: std::mem::replace(&mut seen[1].0, None).unwrap(),
            unique: std::mem::replace(&mut seen[0].0, None).unwrap(),
        },
        _ => Oddity::ManyOfTwoTypes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oddity() {
        let eq = |x: &i32, y: &i32| x == y;
        assert_eq!(
            oddity_by(&mut vec![].into_iter(), &eq),
            Oddity::AllSame);
        assert_eq!(
            oddity_by(&mut vec![1].into_iter(), &eq),
            Oddity::AllSame);
        assert_eq!(
            oddity_by(&mut vec![1, 1].into_iter(), &eq),
            Oddity::AllSame);
        assert_eq!(
            oddity_by(&mut vec![1, 1, 2].into_iter(), &eq),
            Oddity::OneUnlikeOthers { unique: 2, typical: 1});
        assert_eq!(
            oddity_by(&mut vec![2, 1, 1].into_iter(), &eq),
            Oddity::OneUnlikeOthers { unique: 2, typical: 1});
        assert_eq!(
            oddity_by(&mut vec![1, 2, 3].into_iter(), &eq),
            Oddity::MoreThanTwoTypes);
        assert_eq!(
            oddity_by(&mut vec![1, 2, 1, 2].into_iter(), &eq),
            Oddity::ManyOfTwoTypes);
    }
}

fn rec(
        node: &str,
        weights: &HashMap<String, i32>,
        childrens: &HashMap<String, Vec<String>>) -> i32 {
    let mut total_weight = *weights.get(node).unwrap();
    let cs = childrens.get(node);
    let cs = cs.into_iter().flat_map(|cs| cs);
    let o = {
        let cs = cs.map(|c| {
            let w = rec(c, weights, childrens);
            total_weight += w;
            (c, w)
        });
        oddity_by(cs, |&(_, w1), &(_, w2)| w1 == w2)
    };
    match o {
        Oddity::AllSame => {}
        Oddity::OneUnlikeOthers { typical, unique } => {
            let delta = typical.1 - unique.1;
            println!("{}", weights.get(unique.0).unwrap() + delta);
            std::process::exit(0);
        }
        _ => panic!("{:?}", o)
    }
    total_weight
}

fn main() {
    let mut weights = HashMap::<String, i32>::new();
    let mut parents = HashMap::<String, String>::new();
    let mut childrens = HashMap::<String, Vec<String>>::new();

    let stdin = std::io::stdin();
    let re = regex::Regex::new(r"(\w+) \((\d+)\)(?: -> (.*))?").unwrap();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let cap = re.captures(&line).unwrap();
        let name = &cap[1];
        let weight: i32 = cap[2].parse().unwrap();
        weights.insert(name.to_owned(), weight);
        childrens.insert(name.to_owned(), Vec::new());
        let ch = childrens.get_mut(&name.to_owned()).unwrap();
        if let Some(children) = cap.get(3) {
            for child in children.as_str().split(", ") {
                parents.insert(child.to_owned(), name.to_owned());
                ch.push(child.to_owned());
            }
        }
    }

    for name in weights.keys() {
        if !parents.contains_key(name) {
            println!("{}", name);
            rec(name, &weights, &childrens);
        }
    }
}
