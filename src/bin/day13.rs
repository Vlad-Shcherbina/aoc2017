use std::io::{BufRead, BufReader};
fn main() {
    let mut severity = 0;
    for line in BufReader::new(std::io::stdin()).lines() {
        let line = line.unwrap();
        let mut it = line.split(": ");
        let depth = it.next().unwrap();
        let range = it.next().unwrap();
        assert!(it.next().is_none());
        let depth: i32 = depth.parse().unwrap();
        let range: i32 = range.parse().unwrap();
        if depth % (2 * range - 2) == 0 {
            severity += depth * range;
        }
    }
    println!("{}", severity);
}
