use itertools::Itertools;

#[test]
fn day02() {
    let txt = adventofcode2022::get_input(2).unwrap();
    let vs = txt
        .lines()
        .map(|l| {
            (
                match l.as_bytes()[0] {
                    b'A' => 1,
                    b'B' => 2,
                    b'C' => 3,
                    _ => unreachable!(),
                },
                match l.as_bytes()[2] {
                    b'X' => 1,
                    b'Y' => 2,
                    b'Z' => 3,
                    _ => unreachable!(),
                },
            )
        })
        .collect_vec();

    let cnt1: i64 = vs
        .iter()
        .map(|&(a, b)| {
            b + match b - a {
                -2 | 1 => 6,
                -1 | 2 => 0,
                0 => 3,
                _ => unreachable!(),
            }
        })
        .sum();
    dbg!(cnt1);

    let cnt2: i64 = vs
        .iter()
        .map(|&(a, b)| {
            (b - 1) * 3
                + match b {
                    1 => (a + 1) % 3 + 1,
                    2 => a,
                    3 => a % 3 + 1,
                    _ => unreachable!(),
                }
        })
        .sum();
    dbg!(cnt2);
}
