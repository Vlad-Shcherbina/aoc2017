extern crate aoc2017;

const INPUT: &str = "ljoxqyyw";

fn main() {
    let mut result = 0;
    for i in 0..128 {
        for &x in &aoc2017::knot_hash(&format!("{}-{}", INPUT, i)) {
            result += x.count_ones();
        }
    }
    println!("{}", result);
}
