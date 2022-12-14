use itertools::{iproduct, Itertools};

struct Map {
    map: Vec<Vec<u8>>,
    cnt: usize,
}

impl Map {
    fn parse(txt: &str, is_part_two: bool) -> Self {
        let paths: Vec<Vec<(usize, usize)>> = txt
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|t| {
                        t.split(',')
                            .map(|x| x.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_vec()
            })
            .collect_vec();

        let maxy = paths
            .iter()
            .flat_map(|p| p.iter().map(|(y, _)| y))
            .max()
            .unwrap();
        let maxx = paths
            .iter()
            .flat_map(|p| p.iter().map(|(_, x)| x))
            .max()
            .unwrap();

        let mut map = vec![vec![b'.'; maxy + 200 + 1]; maxx + 2 + 1];
        for path in &paths {
            for i in 1..path.len() {
                let ((y0, x0), (y1, x1)) = (path[i - 1], path[i]);
                for (y, x) in iproduct!(y0.min(y1)..=y0.max(y1), x0.min(x1)..=x0.max(x1)) {
                    map[x][y] = b'#';
                }
            }
        }
        if is_part_two {
            for y in 0..map[0].len() {
                map[maxx + 2][y] = b'#';
            }
        }
        Self { map, cnt: 0 }
    }

    fn drop(&mut self) {
        let (mut x, mut y) = (0, 500);
        if self.map[x][y] == b'o' {
            return;
        }
        loop {
            if x + 1 == self.map.len() {
                return;
            }
            match self.map[x + 1][y] {
                b'.' => {
                    x += 1;
                }
                _ => match self.map[x + 1][y - 1] {
                    b'.' => {
                        x += 1;
                        y -= 1;
                    }
                    _ => match self.map[x + 1][y + 1] {
                        b'.' => {
                            x += 1;
                            y += 1;
                        }
                        _ => {
                            self.cnt += 1;
                            self.map[x][y] = b'o';
                            return;
                        }
                    },
                },
            }
        }
    }

    fn sim(&mut self) -> usize {
        let mut cnt = 0;
        loop {
            self.drop();
            if cnt == self.cnt {
                return cnt;
            }
            cnt = self.cnt;
        }
    }
}

#[test]
fn day14() {
    let txt = adventofcode2022::get_input(14).unwrap();
    let mut map = Map::parse(&txt, /*is_part_two=*/ false);
    println!("cnt: {}", map.sim());
    let mut map = Map::parse(&txt, /*is_part_two=*/ true);
    println!("cnt: {}", map.sim());
}
