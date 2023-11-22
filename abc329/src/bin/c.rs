use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        s:Chars,
    }
    let mut cnt = vec![0; 26];
    let rl = run_length_encode(s);
    for (c, ct) in rl {
        cnt[(c as u8 - b'a') as usize] = cnt[(c as u8 - b'a') as usize].max(ct);
    }
    let ans = cnt.into_iter().sum::<usize>();
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
