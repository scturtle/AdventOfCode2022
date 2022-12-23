use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn step(elves: Vec<(i32, i32)>, round: usize) -> (bool, Vec<(i32, i32)>) {
    let cur: HashSet<_> = elves.iter().cloned().collect();
    let mut nxt_cnt: HashMap<(i32, i32), usize> = HashMap::new();
    let dirs: Vec<_> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .cycle()
        .skip(round % 4)
        .take(4)
        .collect();
    let check = |elf: (i32, i32), dir: (i32, i32)| -> bool {
        let nxt_elf = (elf.0 + dir.0, elf.1 + dir.1);
        if dir.0 == 0 {
            !cur.contains(&(nxt_elf.0, nxt_elf.1))
                && !cur.contains(&(nxt_elf.0 + 1, nxt_elf.1))
                && !cur.contains(&(nxt_elf.0 - 1, nxt_elf.1))
        } else {
            !cur.contains(&(nxt_elf.0, nxt_elf.1))
                && !cur.contains(&(nxt_elf.0, nxt_elf.1 + 1))
                && !cur.contains(&(nxt_elf.0, nxt_elf.1 - 1))
        }
    };
    let mut done = true;
    for &elf in &elves {
        if dirs.iter().all(|&dir| check(elf, dir)) {
            continue;
        }
        done = false;
        for &dir in &dirs {
            if check(elf, dir) {
                let nxt_elf = (elf.0 + dir.0, elf.1 + dir.1);
                *nxt_cnt.entry(nxt_elf).or_default() += 1;
                break;
            }
        }
    }
    let mut nxt_elves = vec![];
    for &elf in &elves {
        if dirs.iter().all(|&dir| check(elf, dir)) {
            nxt_elves.push(elf);
            continue;
        }
        let mut moved = false;
        for &dir in &dirs {
            if check(elf, dir) {
                let nxt_elf = (elf.0 + dir.0, elf.1 + dir.1);
                if nxt_cnt[&nxt_elf] == 1 {
                    nxt_elves.push(nxt_elf);
                    moved = true;
                }
                break;
            }
        }
        if !moved {
            nxt_elves.push(elf);
        }
    }
    (done, nxt_elves)
}

#[test]
fn day23() {
    // let txt = adventofcode2022::get_test_input().unwrap();
    let txt = adventofcode2022::get_input(23).unwrap();
    let elves0: Vec<(i32, i32)> = txt
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.bytes().enumerate().filter_map(move |(j, c)| {
                if c == b'#' {
                    Some((i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut elves = elves0.clone();
    for i in 0..10 {
        (_, elves) = step(elves, i);
    }
    let (minx, maxx) = elves.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (miny, maxy) = elves.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    let ans = (maxx - minx + 1) * (maxy - miny + 1) - elves.len() as i32;
    dbg!(ans);

    let mut done;
    let mut elves = elves0;
    for i in 0.. {
        (done, elves) = step(elves, i);
        if done {
            println!("{}", i + 1);
            break;
        }
    }
}
