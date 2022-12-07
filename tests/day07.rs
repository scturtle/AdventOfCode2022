use slab::Slab;

type Handler = usize;

struct File {
    name: String,
    size: usize,
}

struct Dir {
    name: String,
    size: usize,
    items: Vec<(Handler, /*is_dir=*/ bool)>,
}

struct Vfs {
    root: Handler,
    dirs: Slab<Dir>,
    files: Slab<File>,
}

impl Vfs {
    fn new() -> Self {
        Self {
            root: 0,
            dirs: Slab::with_capacity(1024),
            files: Slab::with_capacity(1024),
        }
    }
    fn cd(&self, cur: Handler, name: &str) -> Handler {
        let d = self.dirs.get(cur).unwrap();
        for &(nxt, is_dir) in d.items.iter() {
            if is_dir && name == self.dirs.get(nxt).unwrap().name {
                return nxt;
            }
        }
        panic!();
    }
    fn parse(&mut self, log: &str) {
        self.root = self.dirs.insert(Dir {
            name: "/".to_owned(),
            size: 0,
            items: vec![],
        });
        let mut history = vec![];
        let mut cur: Handler = self.root;
        for line in log.lines() {
            if line.starts_with("$ cd /") {
                history.clear();
                cur = self.root;
            } else if line.starts_with("$ cd ..") {
                cur = history.pop().unwrap();
            } else if line.starts_with("$ cd ") {
                history.push(cur);
                cur = self.cd(cur, &line.trim_end()[5..]);
            } else if line.starts_with("$ ls") {
                let d = self.dirs.get_mut(cur).unwrap();
                d.items.clear();
            } else if line.starts_with("dir ") {
                let name = line.trim_end()[4..].to_owned();
                let ch = self.dirs.insert(Dir {
                    name,
                    size: 0,
                    items: vec![],
                });
                self.dirs.get_mut(cur).unwrap().items.push((ch, true));
            } else {
                let (size, name) = line.trim_end().split_once(' ').unwrap();
                let (size, name) = (size.parse().unwrap(), name.to_owned());
                let f = self.files.insert(File { name, size });
                self.dirs.get_mut(cur).unwrap().items.push((f, false));
            }
        }
    }
    fn calc(&mut self, cur: Handler) {
        let mut size = 0;
        let items = self.dirs.get(cur).unwrap().items.clone();
        for (nxt, is_dir) in items {
            if is_dir {
                self.calc(nxt);
                size += self.dirs.get(nxt).unwrap().size;
            } else {
                size += self.files.get(nxt).unwrap().size;
            }
        }
        self.dirs.get_mut(cur).unwrap().size = size;
    }
    fn show(&self, cur: Handler, indent: usize) {
        if cur == self.root {
            let d = self.dirs.get(cur).unwrap();
            println!("- {} (dir, size={})", d.name, d.size);
        }
        let items = self.dirs.get(cur).unwrap().items.clone();
        for (nxt, is_dir) in items {
            if is_dir {
                let d = self.dirs.get(nxt).unwrap();
                println!("{:>indent$} {} (dir, size={})", '-', d.name, d.size);
                self.show(nxt, indent + 2);
            } else {
                let f = self.files.get(nxt).unwrap();
                println!("{:>indent$} {} (file, size={})", '-', f.name, f.size);
            }
        }
    }
    fn count1(&self, cur: Handler, ans: &mut usize) {
        let d = self.dirs.get(cur).unwrap();
        if d.size <= 100000 {
            *ans += d.size;
        }
        for (nxt, is_dir) in d.items.clone() {
            if is_dir {
                self.count1(nxt, ans);
            }
        }
    }
    fn find2(&self, cur: Handler, best: &mut Handler) {
        let free = 70000000 - self.dirs.get(self.root).unwrap().size;
        let need_to_free = 30000000 - free;
        let best_to_free = self.dirs.get(*best).unwrap().size;
        let d = self.dirs.get(cur).unwrap();
        if d.size >= need_to_free && d.size < best_to_free {
            *best = cur;
        }
        for (nxt, is_dir) in d.items.clone() {
            if is_dir {
                self.find2(nxt, best);
            }
        }
    }
}

#[test]
fn day07() {
    let txt = adventofcode2022::get_input(7).unwrap();
    let mut vfs = Vfs::new();
    vfs.parse(&txt);
    vfs.calc(vfs.root);
    vfs.show(vfs.root, 2);

    let mut ans1 = 0;
    vfs.count1(vfs.root, &mut ans1);
    dbg!(ans1);

    let mut best = vfs.root;
    vfs.find2(vfs.root, &mut best);
    dbg!(vfs.dirs.get(best).unwrap().size);
}
