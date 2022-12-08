#[test]
fn day08() {
    let txt = adventofcode2022::get_input(8).unwrap();
    let m: Vec<Vec<u8>> = txt
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect())
        .collect();
    assert_eq!(m.len(), m[0].len());
    let n = m.len();

    let mut visiable = std::collections::BTreeSet::new();
    let mut max = vec![vec![0; n]; 4];
    for i in 0..n {
        for j in 0..n {
            if i == 0 || m[i][j] > max[0][j] {
                max[0][j] = m[i][j];
                visiable.insert((i, j));
            }
            if i == 0 || m[j][i] > max[1][j] {
                max[1][j] = m[j][i];
                visiable.insert((j, i));
            }
            let i = n - 1 - i;
            if i == n - 1 || m[i][j] > max[2][j] {
                max[2][j] = m[i][j];
                visiable.insert((i, j));
            }
            if i == n - 1 || m[j][i] > max[3][j] {
                max[3][j] = m[j][i];
                visiable.insert((j, i));
            }
        }
    }
    dbg!(visiable.len());

    let mut view = vec![vec![vec![0; n]; n]; 4];
    let mut score = 0;
    for i in 0..n {
        for j in 0..n {
            for ii in (1..=i).rev() {
                view[0][i][j] += 1;
                if m[ii - 1][j] >= m[i][j] {
                    break;
                }
            }
            for jj in (1..=j).rev() {
                view[1][i][j] += 1;
                if m[i][jj - 1] >= m[i][j] {
                    break;
                }
            }
            for ii in i..n - 1 {
                view[2][i][j] += 1;
                if m[ii + 1][j] >= m[i][j] {
                    break;
                }
            }
            for jj in j..n - 1 {
                view[3][i][j] += 1;
                if m[i][jj + 1] >= m[i][j] {
                    break;
                }
            }
            score = score.max((0..4).map(|x| view[x][i][j]).product());
        }
    }
    dbg!(score);
}
