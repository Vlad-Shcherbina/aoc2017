use std::iter::{Iterator, Peekable};

#[derive(Debug)]
enum Node {
    Garbage(i32),
    Group(Vec<Node>),
}

type PeekableCharIter<'a> = Peekable<Box<'a + Iterator<Item=char>>>;

fn parse_garbage(it: &mut PeekableCharIter) -> i32 {
    assert_eq!(it.next().unwrap(), '<');
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

fn parse_group(it: &mut PeekableCharIter) -> Vec<Node> {
    assert_eq!(it.next().unwrap(), '{');
    let mut children = Vec::new();
    if *it.peek().unwrap() == '}' {
        it.next();
        return children;
    }
    children.push(parse_node(it));
    loop {
        match it.next().unwrap() {
            ',' => { children.push(parse_node(it)); }
            '}' => break,
            c => panic!("{}", c)
        }
    }
    children
}

fn parse_node(it: &mut PeekableCharIter) -> Node {
    match *it.peek().unwrap() {
        '<' => Node::Garbage(parse_garbage(it)),
        '{' => Node::Group(parse_group(it)),
        c => panic!("{}", c),
    }
}

fn sum_score(node: &Node, depth: i32) -> i32 {
    match *node {
        Node::Garbage(_) => 0,
        Node::Group(ref ns) => {
            ns.iter().map(|n| sum_score(n, depth + 1)).sum::<i32>() + depth
        }
    }
}

fn sum_garbage(node: &Node) -> i32 {
    match *node {
        Node::Garbage(n) => n,
        Node::Group(ref ns) => {
            ns.iter().map(sum_garbage).sum()
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let it: Box<Iterator<Item=char>> = Box::new(line.chars());
    let mut it = it.peekable();
    let g = parse_node(&mut it);
    for c in it {
        assert!(c.is_whitespace(), "expected eof, got {:?}", c);
    }
    println!("{}", sum_score(&g, 1));
    println!("{}", sum_garbage(&g));
}
