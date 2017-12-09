use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut mem = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let x: i32 = line.parse().unwrap();
        mem.push(x);
    }
    let mut ip: i32 = 0;
    let mut cnt = 0;
    while 0 <= ip && (ip as usize) < mem.len() {
        mem[ip as usize] += 1;
        ip += mem[ip as usize] - 1;
        cnt += 1;
    }
    println!("{}", cnt);
}