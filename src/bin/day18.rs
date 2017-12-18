use std::io::BufRead;

type int = i64;

#[derive(Debug)]
enum Value {
    Const(int),
    Reg(u8),
}

fn parse_reg(s: &str) -> u8 {
    assert_eq!(s.len(), 1);
    s.chars().next().unwrap() as u8 - 'a' as u8
}

impl Value {
    fn parse(s: &str) -> Value {
        match s.parse::<int>() {
            Ok(i) => Value::Const(i),
            Err(_) => Value::Reg(parse_reg(s))
        }
    }

    fn eval(&self, regs: &[int]) -> int {
        match *self {
            Value::Const(c) => c,
            Value::Reg(r) => regs[r as usize],
        }
    }
}

#[derive(Debug)]
enum Instr {
    Snd(Value),
    Set(u8, Value),
    Add(u8, Value),
    Mul(u8, Value),
    Mod(u8, Value),
    Rcv(u8),
    Jgz(Value, Value),
}

impl Instr {
    fn parse(s: &str) -> Instr {
        let mut it = s.split(' ');
        let cmd = it.next().unwrap();
        let instr = match cmd {
            "snd" => Instr::Snd(
                Value::parse(it.next().unwrap())),
            "set" => Instr::Set(
                parse_reg(it.next().unwrap()),
                Value::parse(it.next().unwrap())),
            "add" => Instr::Add(
                parse_reg(it.next().unwrap()),
                Value::parse(it.next().unwrap())),
            "mul" => Instr::Mul(
                parse_reg(it.next().unwrap()),
                Value::parse(it.next().unwrap())),
            "mod" => Instr::Mod(
                parse_reg(it.next().unwrap()),
                Value::parse(it.next().unwrap())),
            "rcv" => Instr::Rcv(
                parse_reg(it.next().unwrap())),
            "jgz" => Instr::Jgz(
                Value::parse(it.next().unwrap()),
                Value::parse(it.next().unwrap())),
            _ => panic!(),
        };
        assert!(it.next().is_none());
        instr
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut program = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        program.push(Instr::parse(&line));
    }

    let mut ip: int = 0;
    let mut regs = vec![0; 26];
    let mut last_snd = None;
    loop {
        match program[ip as usize] {
            Instr::Snd(ref v) => last_snd = Some(v.eval(&regs)),
            Instr::Set(r, ref v) => regs[r as usize] = v.eval(&regs),
            Instr::Add(r, ref v) => regs[r as usize] += v.eval(&regs),
            Instr::Mul(r, ref v) => regs[r as usize] *= v.eval(&regs),
            Instr::Mod(r, ref v) => regs[r as usize] %= v.eval(&regs),
            Instr::Rcv(r) => {
                if regs[r as usize] != 0 {
                    break;
                }
            }
            Instr::Jgz(ref v, ref offset) => {
                if v.eval(&regs) > 0 {
                    ip += offset.eval(&regs) - 1;
                }
            }
        }
        ip += 1;
    }
    println!("{:?}", last_snd);
}
