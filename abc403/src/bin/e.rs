#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        q:usize,
        ts:[(usize, Chars);q],
    }
    let mut trie = Trie::new();
    for i in 0..q {
        if ts[i].0 == 1 {
            trie.insert(ts[i].1.clone(), i);
        } else {
            trie.insert(ts[i].1.clone(), !0);
        }
    }
    let mut ans = vec![0; q];
    for i in 0..q {
        if ts[i].0 == 1 {
            continue;
        }
        let to = trie.search(ts[i].1.clone());
        if i < to {
            ans[i] += 1;
            if to < q {
                ans[to] -= 1;
            }
        }
    }
    for i in 1..q {
        ans[i] += ans[i - 1];
    }
    for i in 0..q {
        println!("{}", ans[i]);
    }
}

pub struct TrieNode {
    next: Vec<usize>,
    _char: char,
    q: usize,
}

impl TrieNode {
    pub fn new(c: char) -> Self {
        TrieNode {
            next: vec![!0; 26],
            _char: c,
            q: !0,
        }
    }
}

pub struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            nodes: vec![TrieNode::new('#')],
        }
    }

    pub fn insert(&mut self, s: Vec<char>, id: usize) {
        let mut node_id = 0;
        for i in 0..s.len() {
            let c = (s[i] as u8 - b'a') as usize;
            let mut next = self.nodes[node_id].next[c];
            if next == !0 {
                next = self.nodes.len();
                self.nodes[node_id].next[c] = next;
                self.nodes.push(TrieNode::new(s[i]));
            }
            node_id = next;
            if i == s.len() - 1 {
                self.nodes[node_id].q = self.nodes[node_id].q.min(id);
            }
        }
    }

    pub fn search(&self, s: Vec<char>) -> usize {
        let mut res = !0;
        let mut node_id = 0;
        for i in 0..s.len() {
            let c = (s[i] as u8 - b'a') as usize;
            let next = self.nodes[node_id].next[c];
            node_id = next;
            res = res.min(self.nodes[node_id].q);
        }
        res
    }
}
