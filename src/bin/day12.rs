use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut adj = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut it = line.split(" <-> ");
        let left = it.next().unwrap();
        let right = it.next().unwrap();
        assert_eq!(left.parse::<usize>().unwrap(), adj.len());
        assert!(it.next().is_none());
        let mut vs = Vec::new();
        for v in right.split(", ") {
            let v: usize = v.parse().unwrap();
            vs.push(v);
        }
        adj.push(vs);
    }

    let mut cnt = 0;
    let mut visited = vec![false; adj.len()];
    for v in 0..adj.len() {
        if visited[v] {
            continue;
        }
        cnt += 1;
        let mut q = vec![v];
        visited[v] = true;
        while !q.is_empty() {
            let v = q.pop().unwrap();
            for &u in &adj[v] {
                if !visited[u] {
                    visited[u] = true;
                    q.push(u);
                }
            }
        }
    }
    println!("{}", cnt);
}
