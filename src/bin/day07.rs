extern crate regex;

use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let mut weights = HashMap::<String, i32>::new();
    let mut parents = HashMap::<String, String>::new();

    let stdin = std::io::stdin();
    let re = regex::Regex::new(r"(\w+) \((\d+)\)(?: -> (.*))?").unwrap();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let cap = re.captures(&line).unwrap();
        println!("{:?}", cap);
        let name = &cap[1];
        let weight: i32 = cap[2].parse().unwrap();
        weights.insert(name.to_owned(), weight);
        for children in cap.get(3) {
            for child in children.as_str().split(", ") {
                parents.insert(child.to_owned(), name.to_owned());
            }
        }
    }

    for name in weights.keys() {
        if !parents.contains_key(name) {
            println!("{}", name);
        }
    }
}
