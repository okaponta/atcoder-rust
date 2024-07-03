 use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut dp = vec![];
    dp.push((0, 0));
    for c in s {
        let c = c.to_digit(10).unwrap() as usize;
        let mut next = vec![];
        for (s, t) in dp {
            next.push((s + t, c));
            next.push((s, t * 10 + c));
        }
        dp = next;
    }
    let ans = dp.into_iter().map(|(s, t)| s + t).sum::<usize>();
    println!("{}", ans / 2);
}