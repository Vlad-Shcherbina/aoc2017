extern crate regex;

use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let re = regex::Regex::new(concat!(
        r"^(?P<reg>\w+) (?P<op>inc|dec) (?P<delta>-?\d+) ",
        r"if (?P<cond_reg>\w+) (?P<cond_op>>|<|>=|<=|==|!=) (?P<cond_value>-?\d+)$"
        )).unwrap();

    let mut regs = HashMap::<String, i32>::new();
    let mut highest = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let cap = re.captures(&line).unwrap();
        println!("{:?}", cap);

        let cond_reg = *regs.get(&cap["cond_reg"]).unwrap_or(&0);
        let cond_value: i32 = cap["cond_value"].parse().unwrap();
        let cond = match &cap["cond_op"] {
            s if s == "<" => cond_reg < cond_value,
            s if s == ">" => cond_reg > cond_value,
            s if s == "<=" => cond_reg <= cond_value,
            s if s == ">=" => cond_reg >= cond_value,
            s if s == "==" => cond_reg == cond_value,
            s if s == "!=" => cond_reg != cond_value,
            _ => panic!()
        };
        if !cond {
            continue;
        }

        let reg = &cap["reg"];
        if !regs.contains_key(reg) {
            regs.insert(reg.to_owned(), 0);
        }
        let reg = regs.get_mut(reg).unwrap();
        let delta: i32 = cap["delta"].parse().unwrap();

        match &cap["op"] {
            s if s == "inc" => { *reg += delta; }
            s if s == "dec" => { *reg -= delta; }
            _ => panic!()
        }
        highest = highest.max(*reg);
    }
    println!("{}", regs.values().max().unwrap());
    println!("{}", highest);
}
