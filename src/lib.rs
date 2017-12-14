#![macro_use]

#[macro_export]
macro_rules! debug {
    ($($x:expr),+) => {
        $(
        print!("{} = {:?}  ", stringify!($x), $x);
        )*
        println!();
    };
}

const N: usize = 256;

pub fn knot_hash(data: &str) -> [u8; 16] {
    let mut result = [0; 16];

    let mut lens = data.to_owned().into_bytes();
    lens.extend(&[17, 31, 73, 47, 23]);
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
        result[i] = x as u8;
    }

    result
}
