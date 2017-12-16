fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let t = line.trim_right().len();
    line.truncate(t);

    let mut names: Vec<char> = (0..16).map(|i| ('a' as u8 + i) as char).collect();
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
    let names: String = names.into_iter().collect();
    println!("{}", names);
}
