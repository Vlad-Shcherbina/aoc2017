use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut adj = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut it = line.split(" <-> ");
        let left = it.next().unwrap();
        let right = it.next().unwrap();
        assert!(it.next().is_none());
        let mut vs = Vec::new();
        for v in right.split(", ") {
            let v: usize = v.parse().unwrap();
            vs.push(v);
        }
        adj.push(vs);
    }

    let mut cnt = 1;
    let mut q = vec![0];
    let mut visited = vec![false; adj.len()];
    visited[0] = true;
    while !q.is_empty() {
        let v = q.pop().unwrap();
        for &u in &adj[v] {
            if !visited[u] {
                visited[u] = true;
                q.push(u);
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
