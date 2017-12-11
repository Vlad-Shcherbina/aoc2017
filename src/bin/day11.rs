use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut x = 0i32;
    let mut y = 0;
    let mut z = 0;
    for step in line.split(',') {
        match step {
            "n"  => { z += 1; }
            "ne" => { y -= 1; }
            "se" => { x += 1; }
            "s"  => { z -= 1; }
            "sw" => { y += 1; }
            "nw" => { x -= 1; }
            _ => panic!("{}", step)
        }
    }
    let ds = [(x - y).abs(), (y - z).abs(), (z - x).abs()];
    let result = ds.into_iter().max().unwrap();
    println!("{}", result);
}
