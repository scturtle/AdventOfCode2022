use itertools::Itertools;
use std::collections::HashMap;

struct Monkey<'a> {
    number: Option<i64>,
    name: &'a str,
    op: char,
    need: [&'a str; 2],
}

impl<'a> Monkey<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            number: None,
            name,
            op: ' ',
            need: ["", ""],
        }
    }
}

impl<'a> std::fmt::Debug for Monkey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.name)?;
        if let Some(n) = self.number {
            write!(f, "{n}")?;
        } else {
            write!(f, "{} {} {}", self.need[0], self.op, self.need[1])?;
        }
        Ok(())
    }
}

#[test]
fn day21() {
    let txt = adventofcode2022::get_input(21).unwrap();
    let mut monkeys: HashMap<&str, Monkey> = txt
        .lines()
        .map(|l| {
            let (name, s) = l.split_once(": ").unwrap();
            let mut monkey = Monkey::new(name);
            if s.as_bytes()[0].is_ascii_digit() {
                monkey.number = Some(s.parse().unwrap());
            } else {
                let (a, op, b) = s.split(' ').collect_tuple().unwrap();
                monkey.need[0] = a;
                monkey.op = op.chars().next().unwrap();
                monkey.need[1] = b;
            }
            (name, monkey)
        })
        .collect();

    let mut need_by: HashMap<&str, Vec<&str>> = HashMap::new();
    for (name, monkey) in &monkeys {
        for need in &monkey.need {
            need_by.entry(need).or_default().push(name);
        }
    }

    // for part two (comment this line for part one)
    monkeys.get_mut("humn").unwrap().number = None;

    let mut q = vec![];
    for m in monkeys.values() {
        if m.number.is_some() {
            q.push(m.name);
        }
    }
    while let Some(a) = q.pop() {
        if let Some(cs) = need_by.get(a) {
            for c in cs {
                let [a, b] = monkeys[c].need;
                if let (Some(x), Some(y)) = (monkeys[a].number, monkeys[b].number) {
                    let n = match monkeys[c].op {
                        '+' => x + y,
                        '-' => x - y,
                        '*' => x * y,
                        '/' => x / y,
                        _ => unreachable!(),
                    };
                    monkeys.get_mut(c).unwrap().number = Some(n);
                    q.push(c);
                }
            }
        }
    }

    // part one
    if let Some(n) = monkeys["root"].number {
        dbg!(n);
        return;
    }

    // part two
    let [a, b] = monkeys["root"].need;
    let (a, b) = if monkeys[a].number.is_some() {
        (a, b)
    } else {
        (b, a)
    };
    monkeys.get_mut(b).unwrap().number = monkeys[a].number;

    let mut q = vec![b];
    while let Some(c) = q.pop() {
        let n = monkeys[c].number.unwrap();
        if c == "humn" {
            dbg!(n);
            break;
        }
        let [a, b] = monkeys[c].need;
        if let Some(x) = monkeys[a].number {
            if monkeys[b].number.is_none() {
                let y = match monkeys[c].op {
                    '+' => n - x,
                    '-' => x - n,
                    '*' => n / x,
                    '/' => x / n,
                    _ => unreachable!(),
                };
                monkeys.get_mut(b).unwrap().number = Some(y);
                q.push(b);
            }
        } else if let Some(y) = monkeys[b].number {
            if monkeys[a].number.is_none() {
                let x = match monkeys[c].op {
                    '+' => n - y,
                    '-' => n + y,
                    '*' => n / y,
                    '/' => n * y,
                    _ => unreachable!(),
                };
                monkeys.get_mut(a).unwrap().number = Some(x);
                q.push(a);
            }
        } else {
            panic!();
        }
    }
}
