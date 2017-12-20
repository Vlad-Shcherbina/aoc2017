use std::io::BufRead;

fn strip_prefix_and_suffix<'a, 'b>(
    s: &'a str,
    prefix: &'b str,
    suffix: &'b str,
) -> Option<&'a str> {
    if s.starts_with(prefix) && s.ends_with(suffix) {
        Some(&s[prefix.len()..s.len() - suffix.len()])
    } else {
        None
    }
}

#[derive(Debug)]
struct Vec3(i32, i32, i32);

fn parse_vec3(s: &str) -> Vec3 {
    let mut it = s.split(',');
    let x: i32 = it.next().unwrap().trim().parse().unwrap();
    let y: i32 = it.next().unwrap().trim().parse().unwrap();
    let z: i32 = it.next().unwrap().trim().parse().unwrap();
    assert!(it.next().is_none());
    Vec3(x, y, z)
}

#[derive(Debug)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

fn parse_particle(line: &str) -> Particle {
    let mut it = line.split(", ");
    let p = parse_vec3(strip_prefix_and_suffix(it.next().unwrap(), "p=<", ">").unwrap());
    let v = parse_vec3(strip_prefix_and_suffix(it.next().unwrap(), "v=<", ">").unwrap());
    let a = parse_vec3(strip_prefix_and_suffix(it.next().unwrap(), "a=<", ">").unwrap());
    assert!(it.next().is_none());
    Particle { p, v, a }
}

fn manhattan(v: &Vec3) -> i32 {
    v.0.abs() + v.1.abs() + v.2.abs()
}

fn main() {
    let stdin = std::io::stdin();
    let mut particles = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        particles.push(parse_particle(&line));
    }
    let i = particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, p)| (manhattan(&p.a), manhattan(&p.v), manhattan(&p.p)))
        .unwrap();
    println!("{:?}", i);
}
