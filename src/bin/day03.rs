#[macro_use]
extern crate aoc2017;

use std::collections::HashMap;

fn spiral_coord(x: i32) -> (i32, i32) {
    if x == 1 {
        return (0, 0);
    }
    let x = x - 1;
    let h = (f64::from(x).sqrt() as i32 + 1) / 2;
    let x = x - (2 * h - 1) * (2 * h - 1) + 1;
    assert!(x >= 1 && x <= 8 * h);
    if x <= 2 * h {
        return (h, x - h);
    }
    if x <= 4 * h {
        return (h - (x - 2 * h), h);
    }
    if x <= 6 * h {
        return (-h, h - (x - 4 * h));
    }
    if x <= 8 * h {
        return (x - 7 * h, -h);
    }
    panic!()
}

fn main() {
    assert_eq!(spiral_coord(1), (0, 0));
    assert_eq!(spiral_coord(12), (2, 1));
    assert_eq!(spiral_coord(23), (0, -2));
    let (x, y) = spiral_coord(1024);
    assert_eq!(x.abs() + y.abs(), 31);
    let (x, y) = spiral_coord(265149);
    debug!(x.abs() + y.abs());

    let mut h: HashMap<(i32, i32), i32> = HashMap::new();
    h.insert((0, 0), 1);
    for i in 2.. {
        let (x, y) = spiral_coord(i);
        let mut s = 0;
        for dx in -1..2 {
            for dy in -1..2 {
                match h.get(&(x + dx, y + dy)) {
                    Some(v) => { s += v; }
                    None => {}
                }
            }
        }
        if s > 265149 {
            debug!(s);
            break;
        }
        h.insert((x, y), s);
    }
}