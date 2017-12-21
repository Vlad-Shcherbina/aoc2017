use std::io::BufRead;
use std::collections::HashMap;

type Grid = Vec<Vec<u8>>;

fn parse(s: &str) -> Grid {
    let mut g = Vec::new();
    for s in s.split('/') {
        g.push(
            s.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!(),
                })
                .collect(),
        );
    }
    g
}

fn transform(g: &Grid, flags: i32) -> Grid {
    let n = g.len();
    (0..n).map(|i| (0..n).map(|j| {
        let mut ii = i;
        let mut jj = j;
        if flags & 1 != 0 {
            ii = n - 1 - ii;
        }
        if flags & 2 != 0 {
            jj = n - 1 - jj;
        }
        if flags & 4 != 0 {
            std::mem::swap(&mut ii, &mut jj);
        }
        g[ii][jj]
    }).collect()).collect()
}

fn sub(g: &Grid, i0: usize, j0: usize, n: usize) -> Grid {
    (0..n).map(|i| g[i0 + i][j0 .. j0 + n].to_vec()).collect()
}

fn put_sub(g: &mut Grid, i0: usize, j0: usize, sg: &Grid) {
    let n = sg.len();
    for i in 0..n {
        for j in 0..n {
            g[i0 + i][j0 + j] = sg[i][j];
        }
    }
}

fn replace(g: &Grid, rules: &HashMap<Grid, Grid>, n1: usize, n2: usize) -> Grid {
    let n = g.len();
    assert_eq!(n % n1, 0);
    let k = n / n1;
    let mut result = vec![vec![0; k * n2]; k * n2];
    for i in 0..k {
        for j in 0..k {
            let t = sub(&g, i * n1, j * n1, n1);
            let t = rules.get(&t).unwrap();
            assert_eq!(t.len(), n2);
            put_sub(&mut result, i * n2, j * n2, t);
        }
    }
    result
}

fn count(g: &Grid) -> i32 {
    let mut cnt = 0;
    for row in g {
        for &e in row {
            cnt += e as i32;
        }
    }
    cnt
}

fn rec(
    g: &Grid, n: i32,
    rules: &HashMap<Grid, Grid>, cache: &mut HashMap<(Grid, i32), i32>,
) -> i32 {
    assert!(n >= 0);
    if n == 0 {
        return count(g);
    }
    if let Some(&x) = cache.get(&(g.clone(), n)) {
        return x;
    }
    let t = replace(g, &rules, 3, 4);
    let t = replace(&t, &rules, 2, 3);
    let t = replace(&t, &rules, 2, 3);
    let mut x = 0;
    for i in 0..3 {
        for j in 0..3 {
            x += rec(&sub(&t, i * 3, j * 3, 3), n - 3, rules, cache);
        }
    }
    cache.insert((g.clone(), n), x);
    x
}

fn main() {
    let stdin = std::io::stdin();
    let mut rules = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut it = line.split(" => ");
        let left = parse(it.next().unwrap());
        let right = parse(it.next().unwrap());
        assert!(it.next().is_none());
        for flags in 0..8 {
            rules.insert(transform(&left, flags), right.clone());
        }
    }
    let start = parse(".#./..#/###");
    let g = replace(&start, &rules, 3, 4);
    let g = replace(&g, &rules, 2, 3);
    let g = replace(&g, &rules, 2, 3);
    let g = replace(&g, &rules, 3, 4);
    let g = replace(&g, &rules, 2, 3);
    println!("{}", count(&g));
    println!("{}", rec(&start, 18, &rules, &mut HashMap::new()));
}
