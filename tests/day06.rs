#[test]
fn day06() {
    let txt = adventofcode2022::get_input(6).unwrap();
    let s = txt.trim().as_bytes();
    let find = |n: usize| -> usize {
        let mut tot = 0;
        let mut cnt = vec![0; 26];
        for i in 0..s.len() {
            if i >= n {
                let idx = (s[i - n] - b'a') as usize;
                cnt[idx] -= 1;
                if cnt[idx] == 0 {
                    tot -= 1;
                }
            }
            {
                let idx = (s[i] - b'a') as usize;
                if cnt[idx] == 0 {
                    tot += 1;
                }
                cnt[idx] += 1;
            }
            if tot == n {
                return i + 1;
            }
        }
        unreachable!()
    };
    dbg!(find(4));
    dbg!(find(14));
}
