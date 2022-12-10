fn next(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dx.abs() <= 1 && dy.abs() <= 1 {
        tail
    } else if dy.abs() == 0 {
        (tail.0 + dx / dx.abs(), tail.1)
    } else if dx.abs() == 0 {
        (tail.0, tail.1 + dy / dy.abs())
    } else {
        (tail.0 + dx / dx.abs(), tail.1 + dy / dy.abs())
    }
}
#[test]
fn day09() {
    let txt = adventofcode2022::get_input(9).unwrap();
    let moves: Vec<(u8, i64)> = txt
        .lines()
        .map(|l| (l.as_bytes()[0], l[2..].parse().unwrap()))
        .collect();
    for n in [2, 10] {
        let mut saw = std::collections::BTreeSet::new();
        let mut rope = vec![(0, 0); n];
        saw.insert((0, 0));
        for &(dir, step) in &moves {
            for _ in 0..step {
                rope[0] = match dir {
                    b'U' => (rope[0].0 - 1, rope[0].1),
                    b'D' => (rope[0].0 + 1, rope[0].1),
                    b'L' => (rope[0].0, rope[0].1 - 1),
                    b'R' => (rope[0].0, rope[0].1 + 1),
                    _ => unreachable!(),
                };
                for i in 1..n {
                    rope[i] = next(rope[i - 1], rope[i]);
                }
                saw.insert(rope.last().cloned().unwrap());
            }
        }
        dbg!(saw.len());
    }
}
