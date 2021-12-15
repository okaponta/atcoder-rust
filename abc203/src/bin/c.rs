use proconio::input;

fn main() {
    input! {
       n:usize,
       mut k:i64,
       mut ab:[(i64,i64);n]
    }
    ab.sort_by(|a, b| a.0.cmp(&(b.0)));
    let mut ans: i64 = 0;
    for i in 0..n {
        let after = k - ab[i].0 + ans;
        if after < 0 {
            println!("{}", ans + k);
            return;
        } else {
            ans = ab[i].0;
            k = after + ab[i].1;
        }
    }
    println!("{}", ans + k);
}
