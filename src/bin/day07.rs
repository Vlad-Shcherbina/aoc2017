extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;

fn rec(
        node: &String,
        weights: &HashMap<String, i32>,
        childrens: &HashMap<String, Vec<String>>) -> i32 {
    let mut s = *weights.get(node).unwrap();
    let mut by_weight = HashMap::<i32, Vec<String>>::new();
    for cs in childrens.get(node) {
        for c in cs {
            let w = rec(c, weights, childrens);
            if !by_weight.contains_key(&w) {
                by_weight.insert(w, Vec::new());
            }
            by_weight.get_mut(&w).unwrap().push(c.clone());
            s += w;
        }
    }
    if by_weight.keys().count() > 1 {
        println!("{} {:?}", node, by_weight);
        for v in by_weight.values() {
            for c in v {
                println!("  {} {:?}", c, weights.get(c));
            }
        }
    }
    s
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
        println!("{:?}", cap);
        let name = &cap[1];
        let weight: i32 = cap[2].parse().unwrap();
        weights.insert(name.to_owned(), weight);
        childrens.insert(name.to_owned(), Vec::new());
        let ch = childrens.get_mut(&name.to_owned()).unwrap();
        for children in cap.get(3) {
            for child in children.as_str().split(", ") {
                parents.insert(child.to_owned(), name.to_owned());
                ch.push(child.to_owned());
            }
        }
    }

    for name in weights.keys() {
        if !parents.contains_key(name) {
            println!("{}", name);
            rec(&name, &weights, &childrens);
        }
    }
}
