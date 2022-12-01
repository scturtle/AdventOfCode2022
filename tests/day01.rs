use itertools::Itertools;

#[test]
fn day01() {
    let txt = adventofcode2022::get_input(1).unwrap();
    let elfs = txt
        .split("\n\n")
        .map(|s| s.lines().flat_map(|t| t.parse::<u64>()).sum::<u64>())
        .collect_vec();
    let cnt1 = elfs.iter().max().unwrap();
    dbg!(cnt1);
    let cnt2 = elfs.iter().sorted().rev().take(3).sum::<u64>();
    dbg!(cnt2);
}
