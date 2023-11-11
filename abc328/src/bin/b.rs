use proconio::input;

fn main() {
    input! {
        n:usize,
        d:[usize;n],
    }
    let mut ans = 0;
    for i in 1..=n {
        let a = is_zoro(i);
        if a == 0 {
            continue;
        }
        for j in 1..=d[i - 1] {
            let b = is_zoro(j);
            if a == b {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn is_zoro(n: usize) -> usize {
    if n < 10 {
        return n;
    }
    if n % 11 == 0 {
        return n / 11;
    }
    0
}
