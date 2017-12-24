fn is_prime(x: i32) -> bool {
    for d in 2.. {
        if x % d == 0 {
            return false;
        }
        if d * d > x {
            break;
        }
    }
    true
}

fn main() {
    let mut a = 1;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    let mut f = 0;
    let mut g = 0;
    let mut h = 0;
    let mut mul_cnt = 0;

    b = 79;
    c = b;
    if a != 0 {
        b *= 100;
        b += 100000;
        c = b;
        c += 17000;
    }
    loop {
        /*f = 1;
        d = 2;
        loop {
            e = 2;
            loop {
                if d * e == b {
                    f = 0;
                }
                e += 1;
                if e == b { break; }
            }
            d += 1;
            if d == b { break; }
        }
        if f == 0 {
            h += 1;
            println!("{}", b);
        }*/
        if !is_prime(b) {
            h += 1;
        }

        if b == c { break; }
        b += 17;
    }
    println!("{}", h);
}
