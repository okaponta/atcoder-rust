use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        q:usize
    }
    let mut linked = LinkedList::from(0, 1 << 60, &a);
    for _ in 0..q {
        input! {c:u8}
        if c == 1 {
            input! {x:usize, y:usize}
            linked.insert_after(x, y);
        } else {
            input! {x:usize}
            linked.remove(x);
        }
    }
    println!("{}", linked.to_vec().iter().join(" "));
}

// 連結リスト
// 要素は異なるようにすること
pub struct LinkedList {
    prev: std::collections::HashMap<usize, usize>,
    next: std::collections::HashMap<usize, usize>,
    first: usize,
    last: usize,
    len: usize,
}

impl LinkedList {
    // firstとlastは変えること！
    pub fn from(first: usize, last: usize, a: &Vec<usize>) -> Self {
        let mut prev = std::collections::HashMap::new();
        let mut next = std::collections::HashMap::new();
        for i in 1..a.len() {
            prev.insert(a[i], a[i - 1]);
            next.insert(a[i - 1], a[i]);
        }
        let len = a.len();
        next.insert(0, a[0]);
        prev.insert(a[0], 0);
        next.insert(a[len - 1], last);
        prev.insert(last, a[len - 1]);
        Self {
            prev,
            next,
            first,
            last,
            len,
        }
    }

    // xの後にyを挿入する
    pub fn insert_after(&mut self, x: usize, y: usize) -> bool {
        if self.prev.get(&x) == None {
            return false;
        }
        let z = self.next[&x];
        self.next.insert(x, y);
        self.prev.insert(y, x);
        self.prev.insert(z, y);
        self.next.insert(y, z);
        self.len += 1;
        true
    }

    pub fn remove(&mut self, x: usize) -> bool {
        if self.prev.get(&x) == None {
            return false;
        }
        let p = self.prev[&x];
        let n = self.next[&x];
        self.prev.insert(n, p);
        self.next.insert(p, n);
        self.len -= 1;
        true
    }

    pub fn first(&self) -> usize {
        self.next[&self.first]
    }

    pub fn last(&self) -> usize {
        self.prev[&self.last]
    }

    pub fn to_vec(&self) -> Vec<usize> {
        let mut c = self.first;
        let mut res = vec![];
        for _ in 0..self.len {
            c = self.next[&c];
            res.push(c);
        }
        res
    }
}
