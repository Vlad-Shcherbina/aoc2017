use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();
    let line = it.next().unwrap().unwrap();
    let line = line.as_bytes();
    let mut sum: i32 = 0;
    assert!(line.len() % 2 == 0);
    for (i, &c) in line.iter().enumerate() {
        assert!(c >= '0' as u8 && c <= '9' as u8);
        let next = line[(i + line.len() / 2) % line.len()];
        if c == next {
            sum += i32::from(c - ('0' as u8));
        }
    }
    println!("{}", sum);
}
