use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut a:[i64;n],
        lrv:[(Usize1,usize,i64);q],
    }
    a.insert(0, 0);
    a.push(0);
    let mut diff = (0..=n)
        .into_iter()
        .map(|i| a[i + 1] - a[i])
        .collect::<Vec<_>>();
    diff[0] = 0;
    diff[n] = 0;
    let mut ans = diff.iter().map(|i| i.abs()).sum::<i64>();
    for (l, r, v) in lrv {
        let bef = diff[l].abs() + diff[r].abs();
        diff[l] += v;
        diff[r] -= v;
        diff[0] = 0;
        diff[n] = 0;
        let after = diff[l].abs() + diff[r].abs();
        ans += after - bef;
        println!("{}", ans);
    }
}
