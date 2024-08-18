use std::{collections::HashMap, mem::swap};

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        mut tp:[(usize,Usize1);m],
        q:usize,
        ab:[(Usize1,Usize1);q],
    }
    let mut t_range = vec![vec![]; n];
    let mut t_ev = vec![vec![]; n];
    tp.sort_by_key(|(t, p)| (*p, *t));
    for i in (0..m).step_by(2) {
        t_ev[tp[i].1].push(tp[i].0);
        t_ev[tp[i + 1].1].push(tp[i + 1].0);
        t_range[tp[i].1].push((tp[i].0, tp[i + 1].0));
    }

    let mut t_s = vec![vec![0]; n];
    for i in 0..n {
        for j in 0..t_range[i].len() {
            let tmp = t_s[i][t_s[i].len() - 1] + t_range[i][j].1 - t_range[i][j].0;
            t_s[i].push(tmp);
        }
    }

    let mut memo = HashMap::new();
    for (mut a, mut b) in ab {
        if let Some(&ans) = memo.get(&(a, b)) {
            println!("{ans}");
        } else {
            if t_range[b].len() < t_range[a].len() {
                swap(&mut a, &mut b);
            }
            let mut ans = 0;

            for range in &t_range[a] {
                let al = range.0;
                let ar = range.1;

                let mut i = t_ev[b].lower_bound(&al);
                let mut j = t_ev[b].lower_bound(&ar);

                if i & 1 == 1 {
                    ans += find_common_range((t_ev[b][i - 1], t_ev[b][i]), (al, ar));
                    i += 1;
                }

                if j & 1 == 1 {
                    if i <= j - 1 {
                        ans += find_common_range((t_ev[b][j - 1], t_ev[b][j]), (al, ar));
                    }
                    j -= 1;
                }

                let i = i >> 1;
                let j = j >> 1;
                if i <= j {
                    ans += t_s[b][j] - t_s[b][i];
                }
            }

            memo.insert((a.min(b), a.max(b)), ans);
            println!("{ans}");
        }
    }
}

fn find_common_range(range1: (usize, usize), range2: (usize, usize)) -> usize {
    let r = range1.1.min(range2.1);
    let l = range1.0.max(range2.0);
    r.saturating_sub(l)
}
