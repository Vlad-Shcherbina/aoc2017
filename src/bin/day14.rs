extern crate aoc2017;
extern crate bit_vec;

const INPUT: &str = "ljoxqyyw";

fn main() {
    let mut grid = Vec::new();
    for i in 0..128 {
        let h = &aoc2017::knot_hash(&format!("{}-{}", INPUT, i));
        grid.push(bit_vec::BitVec::from_bytes(h));
    }

    let mut cnt = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !grid[y][x] {
                continue;
            }
            cnt += 1;
            grid[y].set(x, false);
            let mut q = vec![(x, y)];
            while !q.is_empty() {
                let (x, y) = q.pop().unwrap();
                if y > 0 && grid[y - 1][x] {
                    grid[y - 1].set(x, false);
                    q.push((x, y - 1));
                }
                if y + 1 < grid.len() && grid[y + 1][x] {
                    grid[y + 1].set(x, false);
                    q.push((x, y + 1));
                }
                if x > 0 && grid[y][x - 1] {
                    grid[y].set(x - 1, false);
                    q.push((x - 1, y));
                }
                if x + 1 < grid[y].len() && grid[y][x + 1] {
                    grid[y].set(x + 1, false);
                    q.push((x + 1, y));
                }
            }
        }
    }
    println!("{}", cnt);
}
