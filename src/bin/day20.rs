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

fn int_sqrt(x: i32) -> Option<i32> {
    if x < 0 {
        return None;
    }
    let q = f64::from(x).sqrt().round() as i32;
    if q * q == x {
        Some(q)
    } else {
        None
    }
}

#[allow(unknown_lints)]  // "many_single_char_names" is clippy's
#[allow(many_single_char_names)]
fn roots(a: i32, v: i32, p: i32) -> Option<Vec<i32>> {
    let b = a + 2 * v;
    let c = 2 * p;
    // a*t*t + b*t + c = 0
    if a != 0 {
        let d = b * b - 4 * a * c;
        return match int_sqrt(d) {
            Some(d) =>  Some(
                [-b - d, -b + d].into_iter()
                .filter_map(|x| {
                    if x % (2 * a) == 0 {
                        Some(x / (2 * a))
                    } else {
                        None
                    }
                })
                .collect()),
            None => Some(Vec::new()),
        }
    }
    if b != 0 {
        return if c % b == 0 {
            Some(vec![-c / b])
        } else {
            Some(Vec::new())
        }
    }
    if c != 0 {
        Some(Vec::new())
    } else {
        None
    }
}

fn intersect(xs: Option<Vec<i32>>, ys: Option<Vec<i32>>) -> Option<Vec<i32>> {
    match (xs, ys) {
        (Some(xs), Some(ys)) => {
            let mut result = Vec::new();
            for x in xs {
                if ys.iter().any(|&y| x == y) {
                    result.push(x);
                }
            }
            Some(result)
        }
        (xs, None) => xs.clone(),
        (None, ys) => ys.clone(),
    }
}

fn collide(q: &Particle, r: &Particle) -> Option<i32> {
    let cs = None;
    let cs = intersect(cs, roots(q.a.0 - r.a.0, q.v.0 - r.v.0, q.p.0 - r.p.0));
    let cs = intersect(cs, roots(q.a.1 - r.a.1, q.v.1 - r.v.1, q.p.1 - r.p.1));
    let cs = intersect(cs, roots(q.a.2 - r.a.2, q.v.2 - r.v.2, q.p.2 - r.p.2));
    match cs {
        None => Some(0),
        Some(cs) => {
            for c in cs {
                if c >= 0 {
                    return Some(c);
                }
            }
            None
        }
    }
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

    let mut events = Vec::new();
    for (i, q) in particles.iter().enumerate() {
        for (j, r) in particles[..i].iter().enumerate() {
            match collide(q, r) {
                None => {}
                Some(t) => {
                    events.push((t, i, j));
                }
            }
        }
    }
    events.sort_by(|&(t1, _, _), &(t2, _, _)| t1.partial_cmp(&t2).unwrap());
    let event_groups = group_by_by(
        events.into_iter(),
        |&(t1, _, _), &(t2, _, _)| t1 == t2);
    let mut alive = vec![true; particles.len()];
    for es in event_groups {
        let mut to_kill = Vec::new();
        for (_, i, j) in es {
            if alive[i] && alive[j] {
                to_kill.push(i);
                to_kill.push(j);
            }
        }
        for i in to_kill {
            alive[i] = false;
        }
    }
    let cnt = alive.into_iter().filter(|&x| x).count();
    println!("{}", cnt);
}

struct GroupByBy<T, F, I> {
    it: I,
    group: Vec<T>,
    same: F,
    done: bool,
}

impl<T, F, I> Iterator for GroupByBy<T, F, I>
where
    F: Fn(&T, &T) -> bool,
    I: Iterator<Item=T>,
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.group.is_empty() {
            // only possible on first invocation
            match self.it.next() {
                None => {
                    self.done = true;
                    return None;
                }
                Some(x) => self.group.push(x),
            }
        }
        loop {
            match self.it.next() {
                None => {
                    self.done = true;
                    return Some(std::mem::replace(&mut self.group, Vec::new()));
                }
                Some(x) => {
                    if (self.same)(self.group.last().unwrap(), &x) {
                        self.group.push(x);
                    } else {
                        return Some(std::mem::replace(&mut self.group, vec![x]));
                    }
                }
            }
        }
    }
}

fn group_by_by<I, F>(it: I, same: F) -> GroupByBy<I::Item, F, I>
where
    I: Iterator,
    F: Fn(&I::Item, &I::Item) -> bool,
{
    GroupByBy {it, same, done: false, group: Vec::new()}
}

#[cfg(test)]
mod tests {
    use super::*;
    fn group_by_by_expect(xs: &[i32], expected: &[&[i32]]) {
        let actual: Vec<_> = group_by_by(
            xs.iter().map(|&x| x),
            |&x, &y| x == y).collect();
        let expected: Vec<Vec<_>> = expected.iter().map(|&g| g.to_vec()).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_group_by_by() {
        group_by_by_expect(&[], &[]);
        group_by_by_expect(&[1], &[&[1]]);
        group_by_by_expect(&[1, 1], &[&[1, 1]]);
        group_by_by_expect(&[1, 2], &[&[1], &[2]]);
        group_by_by_expect(&[1, 1, 2, 3, 3], &[&[1, 1], &[2], &[3, 3]]);
    }
}
