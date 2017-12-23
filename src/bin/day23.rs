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

fn main () {
    let stdin = std::io::stdin();
    let mut code = Vec::new();
    for line in stdin.lock().lines() {
        code.push(parse_instr(&line.unwrap()));
    }

    let mut cnt = 0;
    let mut regs = [0i32; 8];
    let mut ip = 0;
    while ip as usize != code.len() {
        match code[ip as usize] {
            Set(r, ref v) => regs[r] = v.eval(&regs),
            Sub(r, ref v) => regs[r] -= v.eval(&regs),
            Mul(r, ref v) => {
                regs[r] *= v.eval(&regs);
                cnt += 1;
            }
            Jnz(ref v, d) => {
                if v.eval(&regs) != 0 {
                    ip += d - 1;
                }
            }
        }
        ip += 1;
    }
    println!("{}", cnt);
}
