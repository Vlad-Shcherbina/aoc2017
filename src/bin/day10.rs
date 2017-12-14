extern crate aoc2017;

const INPUT: &str = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113";

fn main() {
    for &x in &aoc2017::knot_hash(INPUT) {
        print!("{:02x}", x);
    }
    println!();
}
