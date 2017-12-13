extern crate time;

use std::io::{BufRead, BufReader};

fn main() {
    let mut scanners = Vec::<(i32, i32)>::new();
    for line in BufReader::new(std::io::stdin()).lines() {
        let line = line.unwrap();
        let mut it = line.split(": ");
        let depth = it.next().unwrap();
        let range = it.next().unwrap();
        assert!(it.next().is_none());
        scanners.push((depth.parse().unwrap(), range.parse().unwrap()));
    }
    let start = time::precise_time_s();
    for delay in 0.. {
        let mut caught = false;
        for &(depth, range) in &scanners {
            if (depth + delay) % (2 * range - 2) == 0 {
                caught = true;
                break;
            }
        }
        if !caught {
            println!("{}", delay);
            break;
        }
    }
    println!("it took {}", time::precise_time_s() - start);
}
