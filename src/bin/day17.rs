extern crate time;

const STEP: i32 = 363;
const N: usize = 50_000_000;

fn main() {
    let start = time::precise_time_s();

    let mut taken = FenwickTree::<i32>::new(N + 1);
    for i in 0..N + 1 {
        taken.update(i, 1);
    }
    let mut xs = vec![0; N + 1];
    let mut i = N as i32;
    let mut pos = 0;
    while i > 0 {
        xs[pos] = i;
        taken.update(pos, -1);
        let p = taken.range_sum(pos);
        let p = f_mod(p - STEP - 1, i) + 1;
        // TODO: speed up, this is log(N)^2
        pos = bisect_by(0, taken.len() + 1, |q| taken.range_sum(q) < p) - 1;
        i -= 1;
    }
    let t = xs.iter().position(|&x| x == 0).unwrap();
    println!("{}", xs[(t + 1) % xs.len()]);
    println!("it took {}s", time::precise_time_s() - start);
}

fn f_mod(x: i32, y: i32) -> i32 {
    if x >= 0 {
        x % y
    } else {
        y - 1 - (y - 1 - x) % y
    }
}

fn bisect_by<F>(mut lo: usize, mut hi: usize, left: F) -> usize
where
    F: Fn(usize) -> bool,
{
    while hi > lo {
        let mid = lo + (hi - lo) / 2;
        if left(mid) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

use std::ops::AddAssign;

struct FenwickTree<T: AddAssign<T> + Default + Copy> {
    a: Vec<T>,
}

impl<T: AddAssign<T> + Default + Copy> FenwickTree<T> {
    fn new(size: usize) -> Self {
        FenwickTree {
            a: vec![T::default(); size],
        }
    }

    fn len(&self) -> usize {
        self.a.len()
    }

    fn range_sum(&self, mut i: usize) -> T {
        assert!(i <= self.a.len());
        let mut s = T::default();
        while i > 0 {
            s += self.a[i - 1];
            i -= lsb(i);
        }
        s
    }

    fn update(&mut self, mut i: usize, delta: T) {
        assert!(i < self.a.len());
        i += 1;
        while i <= self.a.len() {
            self.a[i - 1] += delta;
            i += lsb(i);
        }
    }
}

fn lsb(x: usize) -> usize {
    x & (1 + !x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fenwick() {
        let mut t = FenwickTree::<i32>::new(5);
        assert_eq!(t.len(), 5);
        t.update(3, 10);
        assert_eq!(t.range_sum(3), 0);
        assert_eq!(t.range_sum(4), 10);
        t.update(2, 20);
        assert_eq!(t.range_sum(2), 0);
        assert_eq!(t.range_sum(3), 20);
        assert_eq!(t.range_sum(4), 30);
        t.update(0, 100);
        t.update(4, 200);
        assert_eq!(t.range_sum(0), 0);
        assert_eq!(t.range_sum(1), 100);
        assert_eq!(t.range_sum(4), 130);
        assert_eq!(t.range_sum(5), 330);
    }
}
