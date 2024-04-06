use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    for i in 0..n {
        let mut ans = 0;
        let mut d = 0;
        for j in 0..n {
            let tmp = (xy[i].0 - xy[j].0) * (xy[i].0 - xy[j].0)
                + (xy[i].1 - xy[j].1) * (xy[i].1 - xy[j].1);
            if d < tmp {
                ans = j + 1;
                d = tmp;
            }
        }
        println!("{}", ans);
    }
}
