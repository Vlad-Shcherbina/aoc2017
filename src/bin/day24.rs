use std::io::BufRead;
use std::collections::HashMap;

fn rec(end: i32, adj: &mut HashMap<i32, HashMap<i32, i32>>) -> (i32, i32) {
    let vs: Vec<i32> = adj[&end].iter()
        .filter_map(|(&v, &cnt)| if cnt > 0 { Some(v) } else { None })
        .collect();
    let mut best = (0, 0);
    for v in vs {
        *adj.get_mut(&end).unwrap().get_mut(&v).unwrap() -= 1;
        *adj.get_mut(&v).unwrap().get_mut(&end).unwrap() -= 1;
        let (length, strength) = rec(v, adj);
        best = best.max((length + 1, strength + end + v));
        *adj.get_mut(&end).unwrap().get_mut(&v).unwrap() += 1;
        *adj.get_mut(&v).unwrap().get_mut(&end).unwrap() += 1;
    }
    best
}

fn main() {
    let stdin = std::io::stdin();
    let mut adj = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut it = line.split('/');
        let left: i32 = it.next().unwrap().parse().unwrap();
        let right: i32 = it.next().unwrap().parse().unwrap();
        assert!(it.next().is_none());
        {
            let e = adj.entry(left).or_insert_with(HashMap::new);
            let ee = e.entry(right).or_insert(0);
            *ee += 1;
        }
        {
            let e = adj.entry(right).or_insert_with(HashMap::new);
            let ee = e.entry(left).or_insert(0);
            *ee += 1;
        }
    }
    println!("{}", rec(0, &mut adj).1);
}
