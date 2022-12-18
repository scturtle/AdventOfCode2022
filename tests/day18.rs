use itertools::{iproduct, Itertools};
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn day18() {
    let txt = adventofcode2022::get_input(18).unwrap();
    let cubes: BTreeSet<[i32; 3]> = txt
        .lines()
        .map(|l| {
            l.split(',')
                .map(|v| v.parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect();

    // part one
    let mut faces = BTreeSet::new();
    for &cube in &cubes {
        for (axis, add) in iproduct!(0..3, [-1, 1].into_iter()) {
            let mut other_cube = cube;
            other_cube[axis] += add;
            if !cubes.contains(&other_cube) {
                // hack: face is [2 * x, 2 * y, 2 * z] for its center
                let mut face = [cube[0] * 2, cube[1] * 2, cube[2] * 2];
                face[axis] += add;
                faces.insert(face);
            }
        }
    }
    dbg!(faces.len());

    // part two
    let mut p = BTreeMap::new();
    for &face in &faces {
        p.insert(face, face);
    }
    fn find(x: [i32; 3], p: &mut BTreeMap<[i32; 3], [i32; 3]>) -> [i32; 3] {
        let px = p[&x];
        if px == x {
            return x;
        }
        let px = find(px, p);
        p.insert(x, px);
        px
    }
    fn union_(x: [i32; 3], y: [i32; 3], p: &mut BTreeMap<[i32; 3], [i32; 3]>) {
        let x = find(x, p);
        let y = find(y, p);
        if x == y {
            return;
        }
        p.insert(y, x);
    }
    let mut union = |x: [i32; 3], y: [i32; 3]| union_(x, y, &mut p);

    for &cube in &cubes {
        for (axis, add, axis2, add2) in
            iproduct!(0..3, [-1, 1].into_iter(), 0..3, [-1, 1].into_iter())
        {
            if axis == axis2 {
                continue;
            }
            let face_base = [cube[0] * 2, cube[1] * 2, cube[2] * 2];
            let mut face = face_base;
            face[axis] += add;
            let mut other_cube = cube;
            other_cube[axis] += add;
            if cubes.contains(&other_cube) {
                continue;
            }
            let mut diag_cube = cube;
            diag_cube[axis] += add;
            diag_cube[axis2] += add2;
            let mut side_cube = cube;
            side_cube[axis2] += add2;
            if cubes.contains(&diag_cube) {
                let mut adj_face = face_base;
                adj_face[axis] += add * 2;
                adj_face[axis2] += add2;
                union(face, adj_face);
            } else if cubes.contains(&side_cube) {
                let mut adj_face = face_base;
                adj_face[axis] += add;
                adj_face[axis2] += add2 * 2;
                union(face, adj_face);
            } else {
                let mut adj_face = face_base;
                adj_face[axis2] += add2;
                union(face, adj_face);
            }
        }
    }

    let mut sizes = BTreeMap::<_, usize>::new();
    for &face in &faces {
        *sizes.entry(find(face, &mut p)).or_default() += 1;
    }
    dbg!(sizes.values().max().unwrap());
}
