use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,
       c:[usize;(1<<n)-1],
    }
    let mut c = c.iter().enumerate().map(|e| (e.0 + 1, e.1)).collect_vec();
    c.sort_by_key(|e| e.1);
    let mut chosen = vec![];
    let mut ans = vec![];
    for (mut i, &c) in c {
        if chosen.len() == n {
            break;
        }
        for e in &chosen {
            i = i.min(i ^ e);
        }
        if i == 0 {
            continue;
        }
        chosen.push(i);
        ans.push(c);
    }
    println!("{:?}", ans.iter().sum::<usize>());
}
