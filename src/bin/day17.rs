const STEP: usize = 363;

fn main() {
    let mut xs = vec![0];
    let mut pos = 0;
    for i in 1..2017 + 1 {
        pos += STEP + 1;
        pos %= xs.len();
        xs.insert(pos, i);
    }
    println!("{}", xs[(pos + 1) % xs.len()]);
}