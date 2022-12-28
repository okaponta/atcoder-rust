use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[Usize1;m],
    }
    let mut tmp = (0..n).into_iter().collect::<Vec<_>>();
    let mut all = (0..n).into_iter().collect::<Vec<_>>();
    let mut rev = (0..n).into_iter().collect::<Vec<_>>();
    for i in 0..m {
        rev.swap(all[a[i]], all[a[i] + 1]);
        all.swap(a[i], a[i] + 1);
    }
    for i in 0..m {
        let s = tmp[a[i]];
        let t = tmp[a[i] + 1];
        println!(
            "{}",
            if s == 0 {
                rev[t] + 1
            } else if t == 0 {
                rev[s] + 1
            } else {
                rev[0] + 1
            }
        );
        tmp.swap(a[i], a[i] + 1);
    }
}
