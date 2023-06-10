use proconio::{fastout, input};
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[i64;n],
        q:usize,
        lr:[(i64,i64);q],
    }
    let mut s = vec![0];
    for i in 1..n {
        if i % 2 == 1 {
            s.push(s[s.len() - 1]);
        } else {
            s.push(s[s.len() - 1] + a[i] - a[i - 1]);
        }
    }
    for (l, r) in lr {
        if l == r {
            println!("0");
            continue;
        }
        let mut diff = 0;
        let lidx = a.upper_bound(&l) - 1;
        let lx = a[lidx];
        if s[lidx + 1] - s[lidx] != 0 {
            diff += l - lx;
        }
        let ridx = a.lower_bound(&r);
        let rx = a[ridx];
        if s[ridx] - s[ridx - 1] != 0 {
            diff += rx - r;
        }
        println!("{}", s[ridx] - s[lidx] - diff);
    }
}
