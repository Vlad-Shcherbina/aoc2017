extern crate regex;

use std::io::BufRead;

fn parse_state(s: &str) -> usize {
    assert_eq!(s.len(), 1);
    (s.bytes().next().unwrap() - b'A') as usize
}

enum Dir {Left, Right}
use Dir::*;

struct Insn {
    value: u8,
    dir: Dir,
    state: usize,
}

fn parse_state_value_block(it: &mut Iterator<Item=String>) -> (usize, Insn) {
    let re = regex::Regex::new(r"  If the current value is ([01]):").unwrap();
    let line = it.next().unwrap();
    let cap = re.captures(&line).unwrap();
    let old_value =cap[1].parse().unwrap();

    let re = regex::Regex::new(r"    - Write the value ([01])\.").unwrap();
    let line = it.next().unwrap();
    let cap = re.captures(&line).unwrap();
    let value: u8 = cap[1].parse().unwrap();

    let re = regex::Regex::new(r"    - Move one slot to the (left|right)\.").unwrap();
    let line = it.next().unwrap();
    let cap = re.captures(&line).unwrap();
    let dir = match &cap[1] {
        "left" => Left,
        "right" => Right,
        _ => panic!()
    };

    let re = regex::Regex::new(r"    - Continue with state (.)\.").unwrap();
    let line = it.next().unwrap();
    let cap = re.captures(&line).unwrap();
    let state = parse_state(&cap[1]);

    (old_value, Insn { value, state, dir })
}

fn main() {
    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines().map(|s| s.unwrap());

    let re = regex::Regex::new(r"Begin in state (.)\.").unwrap();
    let line = it.next().unwrap();
    let cap = re.captures(&line).unwrap();
    let start_state = parse_state(&cap[1]);

    let re = regex::Regex::new(
        r"Perform a diagnostic checksum after (\d+) steps\.").unwrap();
    let line = it.next().unwrap();
    let cap = re.captures(&line).unwrap();
    let num_steps: i32 = cap[1].parse().unwrap();

    let mut program: Vec<[Insn; 2]> = Vec::new();
    loop {
        match it.next() {
            None => break,
            Some(s) => assert_eq!(&s, ""),
        }

        let re = regex::Regex::new(r"In state (.):").unwrap();
        let line = it.next().unwrap();
        let cap = re.captures(&line).unwrap();
        let state = parse_state(&cap[1]);

        let (val, insn0) = parse_state_value_block(&mut it);
        assert_eq!(val, 0);
        let (val, insn1) = parse_state_value_block(&mut it);
        assert_eq!(val, 1);

        assert_eq!(state, program.len());
        program.push([insn0, insn1]);
    }

    let mut tape_left: Vec<u8> = Vec::new();
    let mut tape_right: Vec<u8> = Vec::new();
    let mut cur_value = 0u8;
    let mut state = start_state;
    for _ in 0..num_steps {
        let insn = &program[state][cur_value as usize];
        cur_value = insn.value;
        match insn.dir {
            Left => {
                tape_right.push(cur_value);
                cur_value = tape_left.pop().unwrap_or(0);
            }
            Right => {
                tape_left.push(cur_value);
                cur_value = tape_right.pop().unwrap_or(0);
            }
        }
        state = insn.state;
    }

    let mut cnt = i32::from(cur_value);
    for x in tape_left {
        cnt += i32::from(x);
    }
    for x in tape_right {
        cnt += i32::from(x);
    }
    println!("{}", cnt);
}
