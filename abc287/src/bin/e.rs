use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut si = vec![];
    for i in 0..n {
        si.push((s[i].clone(), i));
    }
    si.sort();
    si.insert(0, (vec!['?'], 0));
    si.push((vec!['?'], 0));
    let mut ans = vec![0; n];
    for i in 1..=n {
        ans[si[i].1] = lcp(&si[i - 1].0, &si[i].0, &si[i + 1].0);
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}

fn lcp(prev: &Vec<char>, cur: &Vec<char>, next: &Vec<char>) -> usize {
    let mut p = 0;
    for i in 0..prev.len().min(cur.len()) {
        if prev[i] == cur[i] {
            p += 1;
        } else {
            break;
        }
    }
    let mut n = 0;
    for i in 0..next.len().min(cur.len()) {
        if next[i] == cur[i] {
            n += 1;
        } else {
            break;
        }
    }
    p.max(n)
}
