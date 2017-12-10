#[macro_use] extern crate aoc2017;

const N: usize = 256;
const INPUT: &str = "120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113";

fn main() {
    let mut lens = INPUT.to_owned().into_bytes();
    lens.extend(&[17, 31, 73, 47, 23]);
    debug!(lens);
    let mut arr: Vec<usize> = (0..N).collect();
    let mut skip = 0;
    let mut start = 0;
    for _round in 0..64 {
        for &len in &lens {
            let len = len as usize;
            for i in 0..len/2 {
                &mut arr.swap((start + i) % N, (start + len - 1 - i) % N);
            }
            start += len + skip;
            skip += 1;
        }
    }
    for i in 0..16 {
        let mut x = 0;
        for j in 0..16 {
            x ^= arr[i * 16 + j];
        }
        print!("{:02x}", x);
    }
    println!();
}