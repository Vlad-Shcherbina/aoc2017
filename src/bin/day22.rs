use std::io::BufRead;

enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}
use State::*;

fn main() {
    let stdin = std::io::stdin();
    let mut grid = std::collections::HashMap::new();

    let mut h = 0;
    let mut w = 0;
    for (i, line) in stdin.lock().lines().enumerate() {
        h += 1;
        let line = line.unwrap();
        w = 0;
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => { grid.insert((j as i32, i as i32), Infected); }
                _ => panic!(),
            }
            w += 1;
        }
    }

    let mut x = w / 2;
    let mut y = h / 2;
    let mut dx = 0;
    let mut dy = -1;
    let mut cnt = 0;
    for _ in 0..10_000_000 {
        let e = grid.entry((x, y)).or_insert(Clean);
        *e = match *e {
            Clean => {
                let t = dy;
                dy = -dx;
                dx = t;
                Weakened
            }
            Weakened => {
                cnt += 1;
                Infected
            }
            Infected => {
                let t = dx;
                dx = -dy;
                dy = t;
                Flagged
            }
            Flagged => {
                dx = -dx;
                dy = -dy;
                Clean
            }
        };
        x += dx;
        y += dy;
    }
    println!("{}", cnt);
}
