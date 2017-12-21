const N: usize = 1_000_000_000;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let t = line.trim_right().len();
    line.truncate(t);

    let mut seen: Vec<Vec<char>> = Vec::new();
    let mut seen_at = std::collections::HashMap::new();
    let mut names: Vec<char> = (0..16).map(|i| (b'a' + i) as char).collect();
    for i in 0..N {
        match seen_at.get(&names) {
            None => {}
            Some(i0) => {
                names = seen[(N - i0) % (i - i0)].clone();
                break;
            }
        }
        seen.push(names.clone());
        seen_at.insert(names.clone(), i);

        for m in line.split(',') {
            let mut it = m.char_indices();
            let (_, first_char) = it.next().unwrap();
            let (i, _) = it.next().unwrap();
            let rest = m.split_at(i).1;
            match first_char {
                's' => {
                    let x: usize = rest.parse().unwrap();
                    let mut q = names[names.len() - x ..].to_vec();
                    q.extend(&names[.. names.len() - x]);
                    names = q;
                }
                'x' => {
                    let mut it = rest.split('/');
                    let a: usize = it.next().unwrap().parse().unwrap();
                    let b: usize = it.next().unwrap().parse().unwrap();
                    assert!(it.next().is_none());
                    names.swap(a, b);
                }
                'p' => {
                    let mut it = rest.chars();
                    let a = it.next().unwrap();
                    assert_eq!(it.next().unwrap(), '/');
                    let b = it.next().unwrap();
                    assert!(it.next().is_none());
                    let pos_a = names.iter().position(|&x| x == a).unwrap();
                    let pos_b = names.iter().position(|&x| x == b).unwrap();
                    names.swap(pos_a, pos_b);
                }
                _ => panic!()
            }
        }
    }
    let names: String = names.into_iter().collect();
    println!("{}", names);
}
