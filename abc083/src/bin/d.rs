use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let r = run_length_encode(s);
    let mut ans = 1;
    let mut sum = 0;
    for (_, i) in r {
        sum += i;
        ans = ans.max(sum.min(n + i - sum));
    }
    println!("{}", ans);
}

fn run_length_encode<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}
