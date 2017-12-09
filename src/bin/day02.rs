use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut s = 0;
    let mut s2 = 0;
    for line in lines {
        let line = line.unwrap();
        let elems: Vec<i32> =
            line.split_whitespace().map(|e| e.parse().unwrap()).collect();
        s += elems.iter().max().unwrap() - elems.iter().min().unwrap();

        let mut div_cnt = 0;
        for (i, &e1) in elems.iter().enumerate() {
            for (j, &e2) in elems.iter().enumerate() {
                if i == j {
                    continue;
                }
                if e1 % e2 == 0 {
                    div_cnt += 1;
                    s2 += e1 / e2;
                }
            }
        }
        assert_eq!(div_cnt, 1);
    }
    println!("{}", s);
    println!("{}", s2);
}