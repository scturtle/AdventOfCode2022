use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    Multiply(i64),
    Plus(i64),
    Square,
}

impl Operation {
    fn on(&self, x: i64) -> i64 {
        match self {
            Operation::Multiply(y) => x * y,
            Operation::Plus(y) => x + y,
            Operation::Square => x * x,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Operation,
    test: i64,
    next: (usize, usize),
    times: usize,
    is_part_one: bool, // or part two
    gcd: i64,
}

impl Monkey {
    fn parse(lines: &str) -> Self {
        let lines: Vec<_> = lines.lines().collect();
        let items: Vec<i64> = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|t| t.parse().unwrap())
            .collect();
        let op: Operation = if lines[2].contains("old * old") {
            Operation::Square
        } else if lines[2].contains("old * ") {
            Operation::Multiply(lines[2].split_once("old * ").unwrap().1.parse().unwrap())
        } else {
            Operation::Plus(lines[2].split_once("old + ").unwrap().1.parse().unwrap())
        };
        let test: i64 = lines[3].split_once(" by ").unwrap().1.parse().unwrap();
        let next: (usize, usize) = (
            lines[4].split_once(" monkey ").unwrap().1.parse().unwrap(),
            lines[5].split_once(" monkey ").unwrap().1.parse().unwrap(),
        );
        Self {
            items,
            op,
            test,
            next,
            times: 0,
            is_part_one: true,
            gcd: 0,
        }
    }

    fn inspect(&mut self) -> Vec<(usize, i64)> {
        let mut throw = vec![];
        self.times += self.items.len();
        for item in self.items.drain(..) {
            let item = self.op.on(item);
            let item = if self.is_part_one {
                item / 3
            } else {
                item % self.gcd
            };
            let next = if item % self.test == 0 {
                self.next.0
            } else {
                self.next.1
            };
            throw.push((next, item));
        }
        throw
    }
}

#[test]
fn day11() {
    let txt = adventofcode2022::get_input(11).unwrap();
    let monkeys0: Vec<Monkey> = txt.split("\n\n").map(Monkey::parse).collect();

    {
        let mut monkeys = monkeys0.clone();
        for _ in 0..20 {
            for i in 0..monkeys.len() {
                for (next, item) in monkeys[i].inspect() {
                    monkeys[next].items.push(item);
                }
            }
        }
        let times: Vec<usize> = monkeys.iter().map(|m| m.times).sorted().rev().collect();
        dbg!(times[0] * times[1]);
    }

    {
        let mut monkeys = monkeys0;
        let gcd = monkeys.iter().map(|m| m.test).product();
        for m in &mut monkeys {
            m.is_part_one = false;
            m.gcd = gcd;
        }
        for _ in 0..10000 {
            for i in 0..monkeys.len() {
                for (next, item) in monkeys[i].inspect() {
                    monkeys[next].items.push(item);
                }
            }
        }
        let times: Vec<usize> = monkeys.iter().map(|m| m.times).sorted().rev().collect();
        dbg!(times[0] * times[1]);
    }
}
