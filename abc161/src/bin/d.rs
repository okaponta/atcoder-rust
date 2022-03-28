use proconio::input;

fn main() {
    input! {
        k:usize,
    }
    let mut ans = 1;
    for _ in 1..k {
        ans = lunlun(ans);
    }
    println!("{}", ans);
}

fn lunlun(v: usize) -> usize {
    if v < 10 {
        return v + 1;
    }
    let first = v % 10;
    let second = (v / 10) % 10;
    if first == 9 || first == second + 1 {
        // くりあがり
        let next_over_sec = lunlun(v / 10);
        let next_first = (next_over_sec % 10).max(1) - 1;
        return next_over_sec * 10 + next_first;
    }
    v + 1
}
