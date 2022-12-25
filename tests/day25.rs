fn to_decimal(s: &[u8]) -> i64 {
    let n = s.len();
    let mut res = 0;
    let mut t = 1;
    for i in (0..n).rev() {
        let v = match s[i] {
            b'2' => 2,
            b'1' => 1,
            b'0' => 0,
            b'-' => -1,
            b'=' => -2,
            _ => unreachable!(),
        };
        res += v * t;
        t *= 5;
    }
    res
}

fn to_snafu(mut n: i64) -> String {
    let mut l = vec![];
    while n > 0 {
        l.push(n % 5);
        n /= 5;
    }
    l.push(0);
    for i in 0..l.len() {
        if l[i] >= 3 {
            l[i + 1] += 1;
            l[i] -= 5;
        }
    }
    while l.last() == Some(&0) {
        l.pop();
    }
    let mut s = String::new();
    for i in (0..l.len()).rev() {
        s.push(match l[i] {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!(),
        });
    }
    s
}

#[test]
fn day25() {
    let txt = adventofcode2022::get_input(25).unwrap();
    let n = txt.lines().map(|l| to_decimal(l.as_bytes())).sum::<i64>();
    println!("{}", to_snafu(n));
}
