use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut ans = 1 << 60;
    for i in 1..=(m.sqrt() + 1).min(n) {
        let div = (m + i - 1) / i;
        if n < div {
            continue;
        }
        ans = ans.min(div * i);
    }
    if ans == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
