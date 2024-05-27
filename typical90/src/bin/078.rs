use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut c = vec![0; n];
    for (a, b) in ab {
        c[a.max(b)] += 1;
    }
    let ans = c.into_iter().filter(|v| v == &1).count();
    println!("{}", ans);
}
