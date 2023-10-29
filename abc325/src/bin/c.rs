use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        mut s:[Chars;h],
    }
    s.insert(0, vec!['.'; w]);
    s.push(vec!['.'; w]);
    for i in 0..h + 2 {
        s[i].insert(0, '.');
        s[i].push('.');
    }
    let mut set = HashSet::new();
    let mut q = VecDeque::new();
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i + 1][j + 1] == '#' && !set.contains(&(i + 1, j + 1)) {
                set.insert((i + 1, j + 1));
                count += 1;
                q.push_back((i + 1, j + 1));
                while let Some((i, j)) = q.pop_front() {
                    for k in 0..3 {
                        for l in 0..3 {
                            if s[i - 1 + k][j - 1 + l] == '#'
                                && !set.contains(&(i - 1 + k, j - 1 + l))
                            {
                                set.insert((i - 1 + k, j - 1 + l));
                                q.push_back((i - 1 + k, j - 1 + l));
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}
