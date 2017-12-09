use std::collections::HashSet;

fn main() {
    // let mut banks = vec![0, 2, 7, 0];
    let mut banks = vec![0, 5, 10, 0, 11, 14, 13, 4, 11, 8, 8, 7, 1, 4, 12, 11];
    let n = banks.len();

    let mut seen = HashSet::new();
    let mut cnt = 0;
    while !seen.contains(&banks) {
        seen.insert(banks.clone());
        let max_idx = (0..n).min_by_key(|&i| -banks[i]).unwrap();
        let t = banks[max_idx];
        banks[max_idx] = 0;
        for i in 0..t {
            banks[(max_idx + i as usize + 1) % n] += 1;
        }
        cnt += 1;
    }
    println!("{}", cnt);
}
