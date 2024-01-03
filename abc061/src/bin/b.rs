use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut ans = vec![0; n];
    for (a, b) in ab {
        ans[a] += 1;
        ans[b] += 1;
    }
    for ans in ans {
        println!("{}", ans);
    }
}
