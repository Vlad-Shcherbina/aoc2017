use std::io::BufRead;
use std::collections::VecDeque;
use std::cell::RefCell;

type Int = i64;

#[derive(Debug)]
enum Value {
    Const(Int),
    Reg(u8),
}

fn parse_reg(s: &str) -> u8 {
    assert_eq!(s.len(), 1);
    s.chars().next().unwrap() as u8 - b'a'
}

impl Value {
    fn parse(s: &str) -> Value {
        match s.parse::<Int>() {
            Ok(i) => Value::Const(i),
            Err(_) => Value::Reg(parse_reg(s))
        }
    }

    fn eval(&self, regs: &[Int]) -> Int {
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

struct State<'a> {
    ip: Int,
    regs: Vec<Int>,
    incoming: &'a RefCell<VecDeque<Int>>,
    outgoing: &'a RefCell<VecDeque<Int>>,
    send_cnt: i32,
}

impl<'a> State<'a> {
    fn new(incoming: &'a RefCell<VecDeque<Int>>,
           outgoing: &'a RefCell<VecDeque<Int>>) -> Self {
        State {
            ip: 0,
            regs: vec![0; 26],
            incoming,
            outgoing,
            send_cnt: 0,
        }
    }

    /// Returns false if blocked
    fn step(&mut self, program: &[Instr]) -> bool {
        match program[self.ip as usize] {
            Instr::Snd(ref v) => {
                self.outgoing.borrow_mut().push_back(v.eval(&self.regs));
                self.send_cnt += 1;
            }
            Instr::Set(r, ref v) => self.regs[r as usize] = v.eval(&self.regs),
            Instr::Add(r, ref v) => self.regs[r as usize] += v.eval(&self.regs),
            Instr::Mul(r, ref v) => self.regs[r as usize] *= v.eval(&self.regs),
            Instr::Mod(r, ref v) => self.regs[r as usize] %= v.eval(&self.regs),
            Instr::Rcv(r) => {
                match self.incoming.borrow_mut().pop_front() {
                    Some(x) => self.regs[r as usize] = x,
                    None => return false,
                }
            }
            Instr::Jgz(ref v, ref offset) => {
                if v.eval(&self.regs) > 0 {
                    self.ip += offset.eval(&self.regs) - 1;
                }
            }
        }
        self.ip += 1;
        true
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut program = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        program.push(Instr::parse(&line));
    }

    let q0to1 = RefCell::new(VecDeque::new());
    let q1to0 = RefCell::new(VecDeque::new());

    let mut s0 = State::new(&q1to0, &q0to1);
    let mut s1 = State::new(&q0to1, &q1to0);
    s1.regs[parse_reg("p") as usize] = 1;

    while s0.step(&program) || s1.step(&program) {}
    println!("{}", s1.send_cnt);
}
