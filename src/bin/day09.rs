use std::iter::Iterator;

#[derive(Debug)]
enum Node {
    Garbage(i32),
    Group(Vec<Node>),
}

fn parse_garbage<I: Iterator<Item=char>>(it: &mut I) -> i32 {
    let mut cnt = 0;
    loop {
        match it.next().unwrap() {
            '!' => { it.next().unwrap(); }
            '>' => break,
            _ => { cnt += 1; }
        }
    }
    cnt
}

fn parse_node<I: Iterator<Item=char>>(it: &mut I) -> Option<Node> {
    match it.next().unwrap() {
        '<' => {
            Some(Node::Garbage(parse_garbage(it)))
        }
        '{' => {
            let mut children = Vec::new();
            match parse_node(it) {
                Some(c) => { children.push(c); }
                None => return Some(Node::Group(children))
            }
            loop {
                match it.next().unwrap() {
                    ',' => {}
                    '}' => break,
                    _ => panic!(),
                }
                children.push(parse_node(it).unwrap());
            }
            Some(Node::Group(children))
        }
        '}' => None,
        c => panic!("{}", c),
    }
}

fn sum_score(node: &Node, depth: i32) -> i32 {
    match node {
        &Node::Garbage(_) => 0,
        &Node::Group(ref ns) => {
            ns.iter().map(|n| sum_score(n, depth + 1)).sum::<i32>() + depth
        }
    }
}

fn sum_garbage(node: &Node) -> i32 {
    match node {
        &Node::Garbage(n) => n,
        &Node::Group(ref ns) => {
            ns.iter().map(sum_garbage).sum()
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let g = parse_node(&mut line.chars()).unwrap();
    println!("{}", sum_score(&g, 1));
    println!("{}", sum_garbage(&g));
}
