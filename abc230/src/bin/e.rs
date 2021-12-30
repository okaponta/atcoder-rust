use proconio::input;

fn main() {
    input! {
       n:i64,
    }
    let mut ans: i64 = 0;
    let mut i: i64 = 0;
    while i * i < n {
        i += 1;
        ans += n / i;
    }
    let mut tmp = n / i;
    ans -= tmp;
    while tmp > 0 {
        let max = n / tmp;
        ans += (max - i + 1) * tmp;
        tmp -= 1;
        i = max + 1;
    }
    println!("{}", ans);
}
