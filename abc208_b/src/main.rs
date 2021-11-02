use proconio::input;

fn main() {
    input! {
            mut p: usize
    }
    let mut coins = vec![1];
    coins.push(1);
    for i in 2..=10 {
        let val = coins[i - 1] * i;
        coins.push(val);
    }
    let mut ans = 0;
    for i in (1..=10).rev() {
        ans += p / coins[i];
        p %= coins[i];
    }
    println!("{}", ans);
}
