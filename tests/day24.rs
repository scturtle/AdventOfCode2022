use std::collections::{HashSet, VecDeque};

type Blizzard = ((i32, i32), (i32, i32));

fn next(blizzards: &[Blizzard], (n, m): (i32, i32)) -> Vec<Blizzard> {
    let mut blizzards: Vec<_> = blizzards.to_vec();
    for b in &mut blizzards {
        b.0 .0 += b.1 .0;
        b.0 .1 += b.1 .1;
        b.0 .0 = if b.0 .0 == 0 {
            n - 2
        } else if b.0 .0 == n - 1 {
            1
        } else {
            b.0 .0
        };
        b.0 .1 = if b.0 .1 == 0 {
            m - 2
        } else if b.0 .1 == m - 1 {
            1
        } else {
            b.0 .1
        };
    }
    blizzards
}

#[test]
fn day24() {
    let txt = adventofcode2022::get_input(24).unwrap();
    let map: Vec<&[u8]> = txt.lines().map(|l| l.as_bytes()).collect();
    let (n, m) = (map.len() as i32, map[0].len() as i32);
    let blizzards: Vec<Blizzard> = map
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter().enumerate().filter_map(move |(j, c)| {
                let dir = match c {
                    b'^' => Some((-1, 0)),
                    b'v' => Some((1, 0)),
                    b'<' => Some((0, -1)),
                    b'>' => Some((0, 1)),
                    _ => None,
                };
                dir.map(|dir| ((i as i32, j as i32), dir))
            })
        })
        .collect();

    let mut state_sets: Vec<HashSet<(i32, i32)>> =
        vec![blizzards.iter().map(|(p, _)| *p).collect()];
    let mut states: Vec<Vec<Blizzard>> = vec![blizzards];

    let mut search = |start_time: usize, start_pos: (i32, i32), end_pos: (i32, i32)| -> usize {
        let mut saw = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((start_time, start_pos));
        saw.insert((start_time, start_pos));
        while let Some((time, pos)) = q.pop_front() {
            if pos == end_pos {
                return time;
            }
            if time + 1 == state_sets.len() {
                states.push(next(states.last().unwrap(), (n, m)));
                state_sets.push(states.last().unwrap().iter().map(|(p, _)| *p).collect());
            }
            let set = &state_sets[time + 1];
            for (ai, aj) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
                let p = (pos.0 + ai, pos.1 + aj);
                if p.0 < 0 || p.0 == n {
                    continue;
                }
                if p != (0, 1)
                    && p != (n - 1, m - 2)
                    && (p.0 == 0 || p.0 == n - 1 || p.1 == 0 || p.1 == m - 1)
                {
                    continue;
                }
                if set.contains(&p) {
                    continue;
                }
                if saw.insert((time + 1, p)) {
                    q.push_back((time + 1, p));
                }
            }
        }
        unreachable!()
    };

    let mut time = 0;
    time = search(time, (0, 1), (n - 1, m - 2));
    dbg!(time);
    time = search(time, (n - 1, m - 2), (0, 1));
    time = search(time, (0, 1), (n - 1, m - 2));
    dbg!(time);
}
