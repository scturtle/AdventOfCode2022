#[derive(Debug)]
struct Node {
    val: i64,
    prev: usize,
    next: usize,
}

#[derive(Debug)]
struct List {
    nodes: Vec<Node>,
}

impl List {
    fn new(numbers: Vec<i64>) -> Self {
        let n = numbers.len();
        let mut nodes = vec![];
        nodes.reserve(n);
        for (idx, val) in numbers.into_iter().enumerate() {
            nodes.push(Node {
                val,
                prev: (idx + n - 1) % n,
                next: (idx + 1) % n,
            });
        }
        Self { nodes }
    }
    fn reorder(&mut self) {
        let n = self.nodes.len();
        for i in 0..n {
            let times = self.nodes[i].val;
            let times = times % (n as i64 - 1);
            let mut j = self.nodes[i].prev;
            self.remove(i); // NOTE: remove it first
            for _ in 0..times.abs() {
                if times > 0 {
                    j = self.nodes[j].next;
                } else {
                    j = self.nodes[j].prev;
                }
            }
            self.insert_after(i, j);
        }
    }
    fn remove(&mut self, i: usize) {
        let prev = self.nodes[i].prev;
        let next = self.nodes[i].next;
        self.nodes[prev].next = next;
        self.nodes[next].prev = prev;
    }
    fn insert_after(&mut self, i: usize, j: usize) {
        let next = self.nodes[j].next;
        self.nodes[i].prev = j;
        self.nodes[i].next = next;
        self.nodes[j].next = i;
        self.nodes[next].prev = i;
    }
    fn get_sum(&self) {
        let mut i = 0;
        while self.nodes[i].val != 0 {
            i += 1;
        }
        let mut sum = 0;
        for j in 1..=3000 {
            i = self.nodes[i].next;
            if j % 1000 == 0 {
                sum += self.nodes[i].val;
            }
        }
        dbg!(sum);
    }
}

#[test]
fn day20() {
    let txt = adventofcode2022::get_input(20).unwrap();
    let numbers: Vec<i64> = txt.lines().map(|l| l.parse().unwrap()).collect();

    // part one
    let mut list = List::new(numbers.clone());
    list.reorder();
    list.get_sum();

    // part two
    let numbers = numbers.into_iter().map(|n| n * 811589153).collect();
    let mut list = List::new(numbers);
    for _ in 0..10 {
        list.reorder();
    }
    list.get_sum();
}
