use itertools::iproduct;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

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

fn dijkstra(mat: &[Vec<u8>], start: Pos, end: Pos) -> usize {
    let (h, w) = (mat.len(), mat[0].len());
    let mut pq = BinaryHeap::new();
    let mut dist = HashMap::new();
    let mut saw = HashSet::new();
    pq.push((Reverse(0), start));
    dist.insert(start, 0);
    while let Some((_, u)) = pq.pop() {
        if u == end {
            return dist[&u];
        }
        if saw.contains(&u) {
            continue;
        }
        saw.insert(u);
        for v in u.around(h, w) {
            // no higher than 1
            if mat[v.0][v.1] > mat[u.0][u.1] + 1 {
                continue;
            }
            if *dist.get(&v).unwrap_or(&usize::MAX) > dist[&u] + 1 {
                dist.insert(v, dist[&u] + 1);
                pq.push((Reverse(dist[&v]), v));
            }
        }
    }
    usize::MAX
}

#[test]
fn day12() {
    let txt = adventofcode2022::get_input(12).unwrap();
    let mut mat: Vec<Vec<u8>> = txt.lines().map(|l| l.bytes().collect()).collect();
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    let (h, w) = (mat.len(), mat[0].len());
    for (i, j) in iproduct!(0..h, 0..w) {
        if mat[i][j] == b'S' {
            start = Pos(i, j);
            mat[i][j] = b'a';
        }
        if mat[i][j] == b'E' {
            end = Pos(i, j);
            mat[i][j] = b'z';
        }
    }
    let (mat, start, end) = (mat, start, end);

    dbg!(dijkstra(&mat, start, end));

    let mut min = usize::MAX;
    for (i, j) in iproduct!(0..h, 0..w) {
        // hack: all b are at column 1
        if j > 2 {
            continue;
        }
        if mat[i][j] == b'a' {
            min = min.min(dijkstra(&mat, Pos(i, j), end));
        }
    }
    dbg!(min);
}
