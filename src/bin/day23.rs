use std::io::BufRead;

enum Val {
    Reg(usize),
    Const(i32),
}
use Val::*;

fn parse_reg(s: &str) -> Option<usize> {
    if s.len() == 1 {
        let c = s.bytes().next().unwrap();
        if c >= b'a' && c <= b'h' {
            return Some((c - b'a') as usize)
        }
    }
    None
}

impl Val {
    fn parse(s: &str) -> Val {
        if let Some(r) = parse_reg(s) {
            return Reg(r);
        }
        Const(s.parse().unwrap())
    }

    fn eval(&self, regs: &[i32]) -> i32 {
        match *self {
            Reg(r) => regs[r],
            Const(c) => c,
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Reg(r) => reg_to_char(r).to_string(),
            Const(c) => c.to_string(),
        }
    }
}

enum Instr {
    Set(usize, Val),
    Sub(usize, Val),
    Mul(usize, Val),
    Jnz(Val, i32),
}
use Instr::*;

fn parse_instr(s: &str) -> Instr {
    let mut it = s.split(' ');
    let cmd = it.next().unwrap();
    let op1 = it.next().unwrap();
    let op2 = it.next().unwrap();
    assert!(it.next().is_none());
    match cmd {
        "set" => Set(parse_reg(op1).unwrap(), Val::parse(op2)),
        "sub" => Sub(parse_reg(op1).unwrap(), Val::parse(op2)),
        "mul" => Mul(parse_reg(op1).unwrap(), Val::parse(op2)),
        "jnz" => Jnz(Val::parse(op1), op2.parse().unwrap()),
        _ => panic!()
    }
}

fn reg_to_char(r: usize) -> char {
    ('a' as u8 + r as u8) as char
}

fn main () {
    let stdin = std::io::stdin();
    let mut code = Vec::new();
    for line in stdin.lock().lines() {
        code.push(parse_instr(&line.unwrap()));
    }

    let mut labels = vec![false; code.len() + 1];
    for (i, instr) in code.iter().enumerate() {
        if let &Jnz(_, d) = instr {
            labels[(i as i32 + d) as usize] = true;
        }
    }

    for (i, instr) in code.iter().enumerate() {
        if labels[i] {
            println!("label_{}:", i);
        }
        match *instr {
            Set(r, ref v) => println!("{} = {}", reg_to_char(r), v.to_string()),
            Sub(r, ref v) => println!("{} -= {}", reg_to_char(r), v.to_string()),
            Mul(r, ref v) => println!("{} *= {}", reg_to_char(r), v.to_string()),
            Jnz(ref v, d) => println!("if {} != 0 goto label_{}", v.to_string(), i as i32 + d),
            Set(r, ref v) => println!("{} = {}", reg_to_char(r), v.to_string()),
            _ => {}
        }
    }
}
