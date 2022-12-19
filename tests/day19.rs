#[allow(clippy::needless_range_loop)]
#[test]
fn day19() {
    let txt = adventofcode2022::get_input(19).unwrap();
    let pat = regex::Regex::new(
        "Blueprint (\\d+): Each ore robot costs (\\d+) ore. \
Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
Each geode robot costs (\\d+) ore and (\\d+) obsidian.",
    )
    .unwrap();

    let blueprints: Vec<(i32, [[i32; 4]; 4])> = txt
        .lines()
        .map(|line| {
            let cap = pat.captures(line).unwrap();
            let cap: Vec<i32> = (1..=7).map(|i| cap[i].parse().unwrap()).collect();
            let mut t = [[0; 4]; 4];
            let id = cap[0];
            t[0][0] = cap[1];
            t[1][0] = cap[2];
            t[2][0] = cap[3];
            t[2][1] = cap[4];
            t[3][0] = cap[5];
            t[3][2] = cap[6];
            (id, t)
        })
        .collect();

    // part one
    // let time_limit = 24;
    // let list = &blueprints[..];

    // part two
    let time_limit = 32;
    let list = &blueprints[..3];

    let mut ans = 0;
    let mut ans2 = 1;
    for &(id, bp) in list {
        let mut q = vec![([1, 0, 0, 0], [0; 4])]; // (rebots, resources)
        for _ in 0..time_limit - 1 {
            let mut next_q = vec![];
            next_q.reserve(8 * q.len());
            for (bot, res) in q {
                for tobi in 0..5 {
                    let mut tob = [0; 4];
                    // `4`: do not build any rebot
                    if tobi != 4 {
                        tob[tobi] = 1;
                    }
                    let mut res = res;
                    for j in 0..3 {
                        res[j] -= (0..4).map(|i| tob[i] * bp[i][j]).sum::<i32>();
                    }
                    if res.iter().any(|&r| r < 0) {
                        continue;
                    }
                    let mut bot = bot;
                    for i in 0..4 {
                        res[i] += bot[i];
                        bot[i] += tob[i];
                    }
                    next_q.push((bot, res));
                }
            }
            // HEURISTIC FTW
            let best_so_far = next_q.iter().map(|(_, res)| res[3]).max().unwrap();
            next_q.retain(|(_, res)| res[3] + 3 >= best_so_far);

            next_q.sort();
            next_q.dedup();
            q = next_q;
        }
        // no need to build robot for the last round
        let best = q.iter().map(|(bot, res)| bot[3] + res[3]).max().unwrap();
        dbg!(id, best);
        ans += id * best;
        ans2 *= best;
    }
    dbg!(ans);
    dbg!(ans2);
}
