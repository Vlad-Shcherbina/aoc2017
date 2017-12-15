const M: i64 = 2147483647;
const MASK: i64 = (1 << 16) - 1;

struct Gen {
    x: i64,
    factor: i64,
}

impl Iterator for Gen {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        self.x *= self.factor;
        self.x %= M;
        Some(self.x)
    }
}

fn main() {
    let  a = Gen {factor: 16807, x: 591};
    let  b = Gen {factor: 48271, x: 393};
    let mut cnt = 0;
    for (x, y) in a.zip(b).take(40_000_000) {
        if x & MASK == y & MASK {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
