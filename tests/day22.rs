use itertools::Itertools;
use std::collections::BTreeMap;

type Direction = (i32, i32);
type BlockId = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Block {
    id: BlockId,
    crs: [[usize; 3]; 4], // [LT, RT, LB, RB]
    width: usize,
}

impl Block {
    fn find_from(&self, i: i32, j: i32) -> ([usize; 3], [usize; 3], Vec<(usize, usize)>) {
        let (i0, i1) = (self.id.0 * self.width, (self.id.0 + 1) * self.width);
        let (j0, j1) = (self.id.1 * self.width, (self.id.1 + 1) * self.width);
        if i0 as i32 - 1 == i {
            (
                self.crs[0],
                self.crs[1],
                (j0..j1).map(|j| (i0, j)).collect(),
            )
        } else if j0 as i32 - 1 == j {
            (
                self.crs[0],
                self.crs[2],
                (i0..i1).map(|i| (i, j0)).collect(),
            )
        } else if i1 as i32 == i {
            (
                self.crs[2],
                self.crs[3],
                (j0..j1).map(|j| (i1 - 1, j)).collect(),
            )
        } else if j1 as i32 == j {
            (
                self.crs[1],
                self.crs[3],
                (i0..i1).map(|i| (i, j1 - 1)).collect(),
            )
        } else {
            unreachable!()
        }
    }
    fn find_to(&self, st: [usize; 3], ed: [usize; 3]) -> Option<(Vec<(usize, usize)>, Direction)> {
        let (i0, i1) = (self.id.0 * self.width, (self.id.0 + 1) * self.width);
        let (j0, j1) = (self.id.1 * self.width, (self.id.1 + 1) * self.width);
        if (st, ed) == (self.crs[0], self.crs[1]) {
            Some(((j0..j1).map(|j| (i0, j)).collect(), (1, 0)))
        } else if (st, ed) == (self.crs[1], self.crs[0]) {
            Some(((j0..j1).map(|j| (i0, j)).rev().collect(), (1, 0)))
        } else if (st, ed) == (self.crs[0], self.crs[2]) {
            Some(((i0..i1).map(|i| (i, j0)).collect(), (0, 1)))
        } else if (st, ed) == (self.crs[2], self.crs[0]) {
            Some(((i0..i1).map(|i| (i, j0)).rev().collect(), (0, 1)))
        } else if (st, ed) == (self.crs[2], self.crs[3]) {
            Some(((j0..j1).map(|j| (i1 - 1, j)).collect(), (-1, 0)))
        } else if (st, ed) == (self.crs[3], self.crs[2]) {
            Some(((j0..j1).map(|j| (i1 - 1, j)).rev().collect(), (-1, 0)))
        } else if (st, ed) == (self.crs[1], self.crs[3]) {
            Some(((i0..i1).map(|i| (i, j1 - 1)).collect(), (0, -1)))
        } else if (st, ed) == (self.crs[3], self.crs[1]) {
            Some(((i0..i1).map(|i| (i, j1 - 1)).rev().collect(), (0, -1)))
        } else {
            None
        }
    }
}

fn turn(dir: Direction, nxt: char) -> Direction {
    match (dir, nxt) {
        ((0, 1), 'R') | ((0, -1), 'L') => (1, 0),
        ((1, 0), 'R') | ((-1, 0), 'L') => (0, -1),
        ((0, -1), 'R') | ((0, 1), 'L') => (-1, 0),
        ((-1, 0), 'R') | ((1, 0), 'L') => (0, 1),
        _ => panic!(),
    }
}

fn password(cur: (usize, usize), dir: Direction) -> usize {
    let n = match dir {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => unreachable!(),
    };
    n + 1000 * (1 + cur.0) + 4 * (1 + cur.1)
}

#[test]
fn day22() {
    let txt = adventofcode2022::get_input(22).unwrap();
    let width = 50;
    let (map, inst_line) = txt.split_once("\n\n").unwrap();
    let mut map: Vec<Vec<u8>> = map.lines().map(|l| l.bytes().collect()).collect();
    let n = map.len();
    let m = map.iter().map(|l| l.len()).max().unwrap();
    for l in &mut map {
        l.resize(m, b' ');
    }
    let insts: Vec<Result<i32, char>> = inst_line
        .trim()
        .bytes()
        .group_by(|c| c.is_ascii_alphabetic())
        .into_iter()
        .map(|(yes, mut cs)| {
            if !yes {
                let cs = cs.collect_vec();
                let n = String::from_utf8_lossy(&cs).parse().unwrap();
                Ok(n)
            } else {
                Err(cs.next().unwrap() as char)
            }
        })
        .collect();

    // part one
    let (mut i, mut j) = (0, (0..).find(|&j| map[0][j] != b' ').unwrap());
    let mut dir: Direction = (0, 1);
    for &inst in &insts {
        match inst {
            Err(nxt) => {
                dir = turn(dir, nxt);
            }
            Ok(step) => {
                for _ in 0..step {
                    let (mut ni, mut nj) = (i as i32, j as i32);
                    (i, j) = loop {
                        (ni, nj) = (ni + dir.0, nj + dir.1);
                        ni = if ni < 0 {
                            n as i32 - 1
                        } else if ni == n as i32 {
                            0
                        } else {
                            ni
                        };
                        nj = if nj < 0 {
                            m as i32 - 1
                        } else if nj == m as i32 {
                            0
                        } else {
                            nj
                        };
                        match map[ni as usize][nj as usize] {
                            b' ' => continue,
                            b'#' => break (i, j),
                            b'.' => break (ni as usize, nj as usize),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
    }
    dbg!(password((i, j), dir));

    // part two
    let mut blocks: BTreeMap<BlockId, Block> = BTreeMap::new();
    // assume the first block's corner TL/TR/BL/BR is (0,0,0)/(0,1,0)/(1,0,0)/(1,1,0)
    let b0 = {
        let id = (0, (0..).find(|&j| map[0][j] != b' ').unwrap() / width);
        Block {
            id,
            crs: [[0, 0, 0], [0, 1, 0], [1, 0, 0], [1, 1, 0]],
            width,
        }
    };
    blocks.insert(b0.id, b0);

    // do dfs and get all other block and its corner
    let mut q = vec![b0.id];
    while let Some(id0) = q.pop() {
        let b0 = blocks[&id0];
        // b's corner are extended from b0 but modified
        let fixed_axis = (0..3)
            .find(|&x| {
                b0.crs[0][x] == b0.crs[1][x]
                    && b0.crs[1][x] == b0.crs[2][x]
                    && b0.crs[2][x] == b0.crs[3][x]
            })
            .unwrap();
        let mut check = |id: BlockId, order: [usize; 4], fix: [usize; 2]| {
            if map[id.0 * width][id.1 * width] != b' ' && !blocks.contains_key(&id) {
                let mut b = Block {
                    id,
                    crs: [[0; 3]; 4],
                    width,
                };
                b.crs = [
                    b0.crs[order[0]],
                    b0.crs[order[1]],
                    b0.crs[order[2]],
                    b0.crs[order[3]],
                ];
                b.crs[fix[0]][fixed_axis] = 1 - b.crs[fix[0]][fixed_axis];
                b.crs[fix[1]][fixed_axis] = 1 - b.crs[fix[1]][fixed_axis];
                q.push(b.id);
                blocks.insert(b.id, b);
            }
        };
        // left block
        if b0.id.1 > 0 {
            let id = (b0.id.0, b0.id.1 - 1);
            check(id, [0, 0, 2, 2], [0, 2]);
        }
        // right block
        if b0.id.1 + 1 < m / width {
            let id = (b0.id.0, b0.id.1 + 1);
            check(id, [1, 1, 3, 3], [1, 3]);
        }
        // up block
        if b0.id.0 > 0 {
            let id = (b0.id.0 - 1, b0.id.1);
            check(id, [0, 1, 0, 1], [0, 1]);
        }
        // down block
        if b0.id.0 + 1 < n / width {
            let id = (b0.id.0 + 1, b0.id.1);
            check(id, [2, 3, 2, 3], [2, 3]);
        }
    }
    assert_eq!(blocks.len(), 6);

    let (mut i, mut j) = (0, (b0.id.1 * width));
    let mut dir: Direction = (0, 1);
    for &inst in &insts {
        match inst {
            Err(nxt) => {
                dir = turn(dir, nxt);
            }
            Ok(step) => {
                for _ in 0..step {
                    ((i, j), dir) = {
                        let (mut next_i, mut next_j) = (i as i32 + dir.0, j as i32 + dir.1);
                        let mut next_dir = dir;
                        // out of boundary, do cube edge search and match
                        if next_i < 0
                            || next_i == n as i32
                            || next_j < 0
                            || next_j == m as i32
                            || map[next_i as usize][next_j as usize] == b' '
                        {
                            let id = (i / width, j / width);
                            let b0 = blocks[&id];
                            let (st, ed, v0) = b0.find_from(next_i, next_j);
                            let v = blocks
                                .values()
                                .filter(|b| b.id != id)
                                .find_map(|b| b.find_to(st, ed))
                                .unwrap();
                            let to = v0
                                .iter()
                                .zip(v.0.iter())
                                .find_map(
                                    |(from, to)| if from == &(i, j) { Some(to) } else { None },
                                )
                                .unwrap();
                            (next_i, next_j) = (to.0 as i32, to.1 as i32);
                            next_dir = v.1;
                        }
                        let (next_i, next_j) = (next_i as usize, next_j as usize);
                        if map[next_i][next_j] == b'#' {
                            ((i, j), dir)
                        } else {
                            assert!(map[next_i][next_j] == b'.');
                            ((next_i, next_j), next_dir)
                        }
                    };
                }
            }
        }
    }
    dbg!(password((i, j), dir));
}
