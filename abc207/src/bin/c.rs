use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:i32,
       mut tlr:[(i32,i32,i32);n]
    }
    let ans = tlr
        .iter()
        .map(|&(t, l, r)| match t {
            2 => (l * 2, r * 2 - 1),
            3 => (l * 2 + 1, r * 2),
            4 => (l * 2 + 1, r * 2 - 1),
            _ => (l * 2, r * 2),
        })
        .tuple_combinations()
        .filter(|&(a, b)| a.0.max(b.0) <= a.1.min(b.1))
        .count();
    println!("{}", ans);
}
