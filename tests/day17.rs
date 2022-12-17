use std::collections::{HashMap, HashSet};

static BLOCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

fn get_basic_blocks() -> Vec<Vec<(i32, i32)>> {
    let mut blocks = vec![];
    for lines in BLOCKS.split("\n\n") {
        let mut block = vec![];
        for (i, l) in lines.lines().rev().enumerate() {
            for (j, c) in l.bytes().enumerate() {
                if c == b'#' {
                    block.push((i as i32, j as i32));
                }
            }
        }
        blocks.push(block);
    }
    blocks
}

fn oppsite(dir: u8) -> u8 {
    match dir {
        b'<' => b'>',
        b'>' => b'<',
        _ => b'^',
    }
}

fn move_block(block: &mut [(i32, i32)], dir: u8) {
    let (ai, aj) = match dir {
        b'<' => (0, -1),
        b'>' => (0, 1),
        b'v' => (-1, 0),
        _ => (1, 0),
    };
    for (i, j) in block {
        *i += ai;
        *j += aj;
    }
}

fn is_ok(block: &[(i32, i32)], chamber: &[[u8; 7]]) -> bool {
    let n = chamber.len() as i32;
    for &(i, j) in block {
        if !(0..7).contains(&j) {
            return false;
        }
        if i < 0 || (i < n && chamber[i as usize][j as usize] != b'.') {
            return false;
        }
    }
    true
}

fn init_block(block: &mut [(i32, i32)], chamber: &[[u8; 7]]) {
    let height = chamber.len() as i32;
    for (i, j) in block {
        *i += height + 3;
        *j += 2;
    }
}

fn rest_block(block: &[(i32, i32)], chamber: &mut Vec<[u8; 7]>) {
    for &(i, j) in block {
        let (i, j) = (i as usize, j as usize);
        while i >= chamber.len() {
            chamber.push(vec![b'.'; 7].try_into().unwrap());
        }
        chamber[i][j] = b'#';
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Pos(usize, usize);
impl Pos {
    pub fn around(&self, h: usize, w: usize) -> impl Iterator<Item = Pos> {
        let (ci, cj) = (self.0 as i32, self.1 as i32);
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .map(move |(i, j)| (ci + i, cj + j))
            .filter(move |&(i, j)| i >= 0 && j >= 0 && i < h as i32 && j < w as i32)
            .map(|(i, j)| Pos(i as usize, j as usize))
    }
}

fn get_surface(chamber: &[[u8; 7]]) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut saw = HashSet::new();
    let mut q = vec![];
    let (n, m) = (chamber.len(), 7);
    for j in 0..m {
        let p = Pos(n - 1, j);
        if chamber[n - 1][j] == b'#' {
            res.push((p.0, p.1));
            saw.insert(p);
        } else {
            q.push(p);
            saw.insert(p);
        }
    }
    while let Some(u) = q.pop() {
        for v in u.around(n, m) {
            if saw.insert(v) {
                if chamber[v.0][v.1] == b'#' {
                    res.push((v.0, v.1));
                } else {
                    q.push(v);
                }
            }
        }
    }
    res.sort();
    let min_i = res[0].0;
    for (i, _) in &mut res {
        *i -= min_i;
    }
    res
}

#[test]
fn day17() {
    let txt = adventofcode2022::get_input(17).unwrap();
    let blocks = get_basic_blocks();
    let mut block_it = blocks.into_iter().enumerate().cycle();
    let mut dir_it = txt.bytes().clone().enumerate().cycle();
    let mut chamber = vec![];
    chamber.reserve(10000);
    let mut saw = HashMap::new();
    let mut result: i64 = 0;
    let mut round: i64 = -1;
    let limit: i64 = 1000000000000;
    while round + 1 < limit {
        round += 1;
        let (block_i, mut block) = block_it.next().unwrap();
        init_block(&mut block, &chamber);
        let dir_i = loop {
            let (dir_i, dir) = dir_it.next().unwrap();
            move_block(&mut block, dir);
            if !is_ok(&block, &chamber) {
                move_block(&mut block, oppsite(dir)); // revert
            }
            move_block(&mut block, b'v');
            if !is_ok(&block, &chamber) {
                move_block(&mut block, oppsite(b'v')); // revert
                rest_block(&block, &mut chamber);
                break dir_i;
            }
        };
        // repeat is still not found
        if result == 0 {
            let height = chamber.len() as i64;
            let key = (block_i, dir_i, get_surface(&chamber));
            // repeat is found, fast forward round
            if let Some((last_round, last_height)) = saw.insert(key, (round, height)) {
                let repeat = (limit - round) / (round - last_round);
                round += repeat * (round - last_round);
                result = repeat * (height - last_height);
            }
        }
        if round + 1 == 2022 {
            dbg!(chamber.len());
        }
    }
    result += chamber.len() as i64;
    dbg!(result);
}
