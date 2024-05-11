use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut trie = Trie::new();
    for (i, s) in s.into_iter().enumerate() {
        trie.insert(s, i);
    }
    let ans = trie
        .nodes
        .iter()
        .map(|n| (n.count - 1) * n.count / 2)
        .sum::<usize>();
    println!("{}", ans - n * (n - 1) / 2);
}

pub struct TrieNode {
    next: Vec<usize>,
    ids: Vec<usize>,
    _char: char,
    count: usize,
}

impl TrieNode {
    pub fn new(c: char) -> Self {
        TrieNode {
            next: vec![!0; 26],
            ids: vec![],
            _char: c,
            count: 0,
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
            self.nodes[node_id].count += 1;
            node_id = next;
        }
        self.nodes[node_id].count += 1;
        self.nodes[node_id].ids.push(id);
    }

    pub fn search(&self, s: Vec<char>) -> bool {
        let mut node_id = 0;
        for i in 0..s.len() {
            let c = (s[i] as u8 - b'a') as usize;
            let next = self.nodes[node_id].next[c];
            if next == !0 {
                return false;
            }
            node_id = next;
        }
        // 途中まで一致の可能性があるので、保持しているidsと照らし合わせ
        0 < self.nodes[node_id].ids.len()
    }

    pub fn start_with(&self, s: Vec<char>) -> bool {
        let mut node_id = 0;
        for i in 0..s.len() {
            let c = (s[i] as u8 - b'a') as usize;
            let next = self.nodes[node_id].next[c];
            if next == !0 {
                return false;
            }
            node_id = next;
        }
        true
    }

    // 要素数
    pub fn size(&mut self) -> usize {
        self.nodes[0].count
    }
}
