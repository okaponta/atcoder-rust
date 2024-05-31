use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        sx:i64,
        sy:i64,
        tx:i64,
        ty:i64,
    }
    let mut ans = vec![];
    f((sx, sy), (tx, ty), &mut ans);
    f((tx, ty), (sx, sy), &mut ans);
    ans.push('L');
    f((sx - 1, sy), (tx, ty + 1), &mut ans);
    ans.push('D');
    ans.push('R');
    f((tx + 1, ty), (sx, sy - 1), &mut ans);
    ans.push('U');
    println!("{}", ans.iter().join(""));
}

fn f(s: (i64, i64), t: (i64, i64), ans: &mut Vec<char>) {
    if s.0 + s.1 < t.0 + t.1 {
        (0..(t.1 - s.1)).into_iter().for_each(|_i| ans.push('U'));
        (0..(t.0 - s.0)).into_iter().for_each(|_i| ans.push('R'));
    } else {
        (0..(s.1 - t.1)).into_iter().for_each(|_i| ans.push('D'));
        (0..(s.0 - t.0)).into_iter().for_each(|_i| ans.push('L'));
    }
}
