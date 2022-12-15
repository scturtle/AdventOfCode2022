use itertools::Itertools;

#[test]
fn day15() {
    let txt = adventofcode2022::get_input(15).unwrap();
    let pat = regex::Regex::new("x=([^,]+), y=([^:]+)").unwrap();
    let pairs: Vec<((i64, i64), (i64, i64))> = txt
        .lines()
        .map(|l| {
            pat.captures_iter(l)
                .map(|m| (m[1].parse().unwrap(), m[2].parse().unwrap()))
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    let count = |ty: i64| -> i64 {
        let mut segs = vec![];
        for &((sx, sy), (bx, by)) in &pairs {
            let dis = (sx - bx).abs() + (sy - by).abs();
            let rest = dis - (sy - ty).abs();
            if rest >= 0 {
                segs.push((sx - rest, sx + rest));
            }
        }
        segs.sort();
        let mut sum = 0;
        let mut last = i64::MIN;
        for &(st, ed) in &segs {
            if st <= last {
                if ed > last {
                    sum += ed - last;
                    last = ed;
                }
            } else {
                // for part two
                if last != i64::MIN && last + 1 != st {
                    let tx = last + 1;
                    if (0..=4000000).contains(&tx) {
                        assert_eq!(tx + 1, st);
                        println!("ans = {}", tx * 4000000 + ty);
                    }
                }
                sum += ed - st + 1;
                last = ed;
            }
        }
        sum
    };

    // part one
    let ty = 2000000;
    let mut sum = count(ty);
    sum -= pairs
        .iter()
        .filter_map(|&(_, (bx, by))| if by == ty { Some((bx, by)) } else { None })
        .unique()
        .count() as i64;
    dbg!(sum);

    // part two
    for ty in 0..=4000000 {
        count(ty);
    }
}
