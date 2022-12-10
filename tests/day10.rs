#[test]
fn day10() {
    let txt = adventofcode2022::get_input(10).unwrap();
    let cmds: Vec<Option<i32>> = txt
        .lines()
        .map(|l| {
            if &l[..4] == "addx" {
                l[5..].parse().ok()
            } else {
                None
            }
        })
        .collect();

    {
        let mut x = 1;
        let mut cycle = 0;
        let mut res = 0;
        let mut biu = |v: i32| {
            cycle += 1;
            if cycle % 40 == 20 {
                res += cycle * x;
            }
            x += v;
        };
        for cmd in &cmds {
            if let &Some(v) = cmd {
                biu(0);
                biu(v);
            } else {
                biu(0);
            }
        }
        dbg!(res);
    }

    {
        let mut buf = vec![vec![b'.'; 40]; 6];
        let mut x = 1;
        let (mut i, mut j) = (0, 0);
        let mut biu = |v: i32| {
            if x - 1 <= j && j <= x + 1 {
                buf[i][j as usize] = b'#';
            }
            j += 1;
            if j == 40 {
                j = 0;
                i += 1;
            }
            x += v;
        };
        for cmd in &cmds {
            if let &Some(v) = cmd {
                biu(0);
                biu(v);
            } else {
                biu(0);
            }
        }
        for line in buf {
            println!("{}", String::from_utf8_lossy(&line));
        }
    }
}
