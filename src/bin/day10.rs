// const N: usize = 5;
// const LENS: &[usize] = &[3, 4, 1, 5];
const N: usize = 256;
const LENS: &[usize] = &[120,93,0,90,5,80,129,74,1,165,204,255,254,2,50,113];

fn main() {
    let mut arr: Vec<usize> = (0..N).collect();
    let mut skip = 0;
    let mut start = 0;
    for &len in LENS {
        for i in 0..len/2 {
            &mut arr.swap((start + i) % N, (start + len - 1 - i) % N);
        }
        start += len + skip;
        skip += 1;
    }
    println!("{}", arr[0] * arr[1]);
}