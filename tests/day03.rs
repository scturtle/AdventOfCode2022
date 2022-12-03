use itertools::Itertools;

fn to_priority(b: u8) -> i64 {
    (if b.is_ascii_lowercase() {
        b - b'a' + 1
    } else {
        b - b'A' + 27
    }) as i64
}

#[test]
fn day03() {
    let txt = adventofcode2022::get_input(3).unwrap();
    let rucksacks = txt
        .lines()
        .map(|l| l.trim().bytes().map(to_priority).collect_vec())
        .collect_vec();

    let cnt1: i64 = rucksacks
        .iter()
        .map(|r| {
            let (a, b) = r.split_at(r.len() / 2);
            a.iter().filter(|t| b.contains(t)).next().unwrap()
        })
        .sum();
    dbg!(cnt1);

    let cnt2: i64 = rucksacks
        .chunks(3)
        .map(|g| {
            g[0].iter()
                .filter(|t| g[1].contains(t) && g[2].contains(t))
                .next()
                .unwrap()
        })
        .sum();
    dbg!(cnt2);
}
