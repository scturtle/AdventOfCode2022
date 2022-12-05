use itertools::Itertools;

#[test]
fn day05() {
    let txt = adventofcode2022::get_input(5).unwrap();
    let mut stacks: Vec<Vec<u8>> = vec![vec![]; 10];
    let mut it = txt.lines();
    for line in &mut it {
        if line.trim().is_empty() {
            break;
        }
        for (i, c) in line.bytes().enumerate() {
            if (i + 3) % 4 == 0 && c.is_ascii_uppercase() {
                stacks[1 + (i - 1) / 4].insert(0, c);
            }
        }
    }

    let steps: Vec<(usize, usize, usize)> = it
        .map(|l| {
            l.replace("move ", "")
                .replace("from ", "")
                .replace("to ", "")
                .split(' ')
                .map(|t| t.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let mut stacks1 = stacks.clone();
    let mut stacks2 = stacks.clone();
    let mut tmp = vec![];
    for &(c, i, j) in &steps {
        for _ in 0..c {
            let t = stacks1[i].pop().unwrap();
            stacks1[j].push(t);
        }
        for _ in 0..c {
            tmp.push(stacks2[i].pop().unwrap());
        }
        for _ in 0..c {
            stacks2[j].push(tmp.pop().unwrap());
        }
    }
    let cvt = |stacks: Vec<Vec<u8>>| {
        String::from_utf8_lossy(
            &stacks
                .iter()
                .filter_map(|s| s.last().cloned())
                .collect_vec(),
        )
        .to_string()
    };
    dbg!(cvt(stacks1));
    dbg!(cvt(stacks2));
}
