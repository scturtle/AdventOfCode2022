use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::Debug;

enum Packet {
    Int(i64),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(bs: &[u8]) -> Self {
        if bs[0].is_ascii_digit() {
            Packet::Int(String::from_utf8_lossy(bs).parse().expect("digits"))
        } else {
            assert_eq!(bs[0], b'[');
            let mut lst = vec![];
            let mut i = 1;
            while i < bs.len() {
                let mut j = i;
                let mut p = 0;
                loop {
                    match bs[j] {
                        b',' if p == 0 => break,
                        b']' if p == 0 => break,
                        b'[' => p += 1,
                        b']' => p -= 1,
                        _ => {}
                    }
                    j += 1;
                }
                if i < j {
                    lst.push(Packet::parse(&bs[i..j]));
                }
                i = j + 1;
            }
            Packet::List(lst)
        }
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(val) => write!(f, "{val}"),
            Packet::List(lst) => {
                write!(f, "[")?;
                for (i, t) in lst.iter().enumerate() {
                    write!(f, "{t:?}")?;
                    if i + 1 < lst.len() {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(_), Packet::Int(r)) => self.cmp(&Packet::List(vec![Packet::Int(*r)])),
            (Packet::Int(l), Packet::List(_)) => Packet::List(vec![Packet::Int(*l)]).cmp(other),
            (Packet::List(l), Packet::List(r)) => {
                let n = l.len().max(r.len());
                for i in 0..n {
                    if i == l.len() {
                        return Ordering::Less;
                    }
                    if i == r.len() {
                        return Ordering::Greater;
                    }
                    let res = l[i].cmp(&r[i]);
                    if res != Ordering::Equal {
                        return res;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

#[test]
fn day13() {
    let txt = adventofcode2022::get_input(13).unwrap();
    let pairs: Vec<(Packet, Packet)> = txt
        .split("\n\n")
        .map(|ls| {
            ls.lines()
                .map(|l| Packet::parse(l.as_bytes()))
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut sum = 0;
    for (i, (a, b)) in pairs.iter().enumerate() {
        if a < b {
            sum += i + 1;
        }
    }
    dbg!(sum);

    let pkts: Vec<Packet> = pairs
        .into_iter()
        .flat_map(|(a, b)| [a, b].into_iter())
        .sorted()
        .collect();
    let a = Packet::parse(b"[[2]]");
    let b = Packet::parse(b"[[6]]");
    let pos_a = pkts.binary_search(&a).unwrap_err();
    let pos_b = pkts.binary_search(&b).unwrap_err();
    dbg!((pos_a + 1) * (pos_b + 2));
}
