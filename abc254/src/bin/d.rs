use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut maxsq = vec![1; n + 1];
    for i in 2..=n.sqrt() {
        for j in (i * i..=n).step_by(i * i) {
            maxsq[j] = i * i;
        }
    }
    let mut count = vec![0; n + 1];
    for i in 1..=n {
        count[i / maxsq[i]] += 1;
    }
    let mut ans = 0;
    for i in 1..=n {
        ans += count[i] * count[i];
    }
    println!("{}", ans);
}
