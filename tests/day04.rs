#[test]
fn day04() {
    let txt = adventofcode2022::get_input(4).unwrap();
    let vs: Vec<Vec<i64>> = txt
        .lines()
        .map(|l| {
            l.split(|c| c == ',' || c == '-')
                .map(|t| t.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let cnt1: usize = vs
        .iter()
        .map(|a| ((a[0] <= a[2] && a[3] <= a[1]) || (a[2] <= a[0] && a[1] <= a[3])) as usize)
        .sum();
    dbg!(cnt1);

    let cnt2: usize = vs
        .iter()
        .map(|a| (a[0].max(a[2]) <= a[1].min(a[3])) as usize)
        .sum();
    dbg!(cnt2);
}
