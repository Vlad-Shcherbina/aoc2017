use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    let mut cnt = 0;
    for line in lines {
        let line = line.unwrap();
        let words = line.split_whitespace();
        let mut used = HashSet::new();
        let mut valid = true;
        for word in words {
            let mut chars: Vec<_> = word.chars().collect();
            chars.sort();
            let word: String = chars.into_iter().collect();
            if used.contains(&word) {
                valid = false;
                break;
            }
            used.insert(word);
        }
        if valid {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}