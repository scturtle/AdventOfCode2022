use itertools::Itertools;
use std::collections::{BTreeMap, HashSet};

type Valve = u16;

struct Solution {
    stops: Vec<(Valve, i32)>,
    flowrate: BTreeMap<Valve, i32>,
    dist: BTreeMap<(Valve, Valve), i32>,
}

fn to_valve(s: &str) -> Valve {
    (s.as_bytes()[0] - b'A') as u16 * 100 + (s.as_bytes()[1] - b'A') as u16
}

impl Solution {
    fn parse(txt: &str) -> Self {
        let mut flowrate = BTreeMap::new();
        let mut dist = BTreeMap::new();
        let pat = regex::Regex::new(
            "Valve (.{2}) has flow rate=([0-9]+); tunnels? leads? to valves? (.+)",
        )
        .unwrap();
        for line in txt.lines() {
            let cap = pat.captures(line).unwrap();
            let u = to_valve(&cap[1]);
            flowrate.insert(u, cap[2].parse().unwrap());
            dist.insert((u, u), 0);
            for v in cap[3].split(", ").map(to_valve) {
                dist.insert((u, v), 1);
                dist.insert((v, u), 1);
            }
        }
        let mut stops = vec![];
        for (&v, &r) in &flowrate {
            if r > 0 {
                stops.push((v, r));
            }
        }
        Self {
            stops,
            flowrate,
            dist,
        }
    }

    fn floyd_warshall(&mut self) {
        let valves = self.flowrate.keys().cloned().collect_vec();
        for &k in &valves {
            for &i in &valves {
                for &j in &valves {
                    if !self.dist.contains_key(&(i, k)) || !self.dist.contains_key(&(k, j)) {
                        continue;
                    }
                    let new_dist = self.dist[&(i, k)] + self.dist[&(k, j)];
                    if !self.dist.contains_key(&(i, j)) || self.dist[&(i, j)] > new_dist {
                        self.dist.insert((i, j), new_dist);
                    }
                }
            }
        }
    }

    fn dfs(&self, u: Valve, rest: i32, sofar: i32, saw: &mut HashSet<Valve>, best: &mut i32) {
        for &(v, r) in &self.stops {
            if saw.contains(&v) {
                continue;
            }
            let dist = self.dist[&(u, v)];
            let new_rest = rest - dist - 1;
            if new_rest <= 0 {
                continue;
            }
            saw.insert(v);
            let new_sofar = sofar + r * new_rest;
            *best = new_sofar.max(*best);
            self.dfs(v, new_rest, new_sofar, saw, best);
            saw.remove(&v);
        }
    }

    fn search(&self) -> i32 {
        let mut best = 0;
        let mut saw = HashSet::new();
        self.dfs(to_valve("AA"), 30, 0, &mut saw, &mut best);
        best
    }

    fn dfs2(
        &self,
        you: (i32, Valve),
        other: (i32, Valve),
        rest: i32,
        sofar: i32,
        saw: &mut HashSet<Valve>,
        best: &mut i32,
    ) {
        if sofar == 0 {
            // dedup at the start
            if you.0 > other.0 && other.0 > 0 {
                return;
            }
            // heuristic: do not go for the low valves at the start
            if (self.flowrate[&you.1] > 0 && self.flowrate[&you.1] < 10)
                || (self.flowrate[&other.1] > 0 && self.flowrate[&other.1] < 10)
            {
                return;
            }
        }

        // dedup by swapping
        if you.0 > other.0 {
            self.dfs2(other, you, rest, sofar, saw, best);
            return;
        }

        // time elapsed
        let (elapsed, u) = you;
        if rest <= elapsed {
            return;
        }
        let rest = rest - elapsed;
        let other = (other.0 - elapsed, other.1);
        let sofar = sofar + rest * self.flowrate[&u];

        for &(v, _) in &self.stops {
            if saw.contains(&v) {
                continue;
            }
            let dist = self.dist[&(u, v)] + 1;
            if dist >= rest {
                continue;
            }
            saw.insert(v);
            self.dfs2((dist, v), other, rest, sofar, saw, best);
            saw.remove(&v);
        }

        // if one cannot find the next stop to go
        if rest > other.0 {
            let sofar = sofar + (rest - other.0) * self.flowrate[&other.1];
            *best = sofar.max(*best);
        }
    }

    fn search2(&self) -> i32 {
        let mut best = 0;
        let mut saw = HashSet::new();
        self.dfs2(
            (0, to_valve("AA")),
            (0, to_valve("AA")),
            26,
            0,
            &mut saw,
            &mut best,
        );
        best
    }
}

#[test]
fn day16() {
    let txt = adventofcode2022::get_input(16).unwrap();
    let mut it = Solution::parse(&txt);
    it.floyd_warshall();
    dbg!(it.search());
    dbg!(it.search2());
}
