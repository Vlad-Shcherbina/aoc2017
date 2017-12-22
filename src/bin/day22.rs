use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut infected = std::collections::HashSet::new();

    let mut h = 0;
    let mut w = 0;
    for (i, line) in stdin.lock().lines().enumerate() {
        h += 1;
        let line = line.unwrap();
        w = 0;
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => { infected.insert((j as i32, i as i32)); }
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
    for _ in 0..10000 {
        if infected.insert((x, y)) {
            let t = dy;
            dy = -dx;
            dx = t;
            cnt += 1;
        } else {
            let t = dx;
            dx = -dy;
            dy = t;
            infected.remove(&(x, y));
        }
        x += dx;
        y += dy;
    }
    println!("{}", cnt);
}
