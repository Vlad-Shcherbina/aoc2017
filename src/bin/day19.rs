use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<_> = stdin.lock().lines()
        .map(|line| line.unwrap().into_bytes()).collect();

    let mut x = lines[0].iter().position(|&c| c == '|' as u8).unwrap() as i32;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = 1;

    let mut cnt = 0;
    let mut s = String::new();
    loop {
        match lines[y as usize][x as usize] as char {
            ' ' => break,
            '+' => {
                let t = dx;
                dx = dy;
                dy = -t;
                if lines[(y + dy) as usize][(x + dx) as usize] == ' ' as u8 {
                    dx = -dx;
                    dy = -dy;
                }
            }
            '|' | '-' => {}
            c => s.push(c),
        }
        x += dx; y += dy;
        cnt += 1;
    }
    println!("{} {}", s, cnt);
}