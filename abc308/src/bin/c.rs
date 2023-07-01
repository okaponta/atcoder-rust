use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n],
    }
    let mut v = ab
        .into_iter()
        .enumerate()
        .map(|(i, (a, b))| (a, b, i + 1))
        .collect::<Vec<_>>();
    v.sort_by(|a, b| ((b.0 * (a.0 + a.1)).cmp(&(a.0 * (b.0 + b.1)))).then(a.2.cmp(&b.2)));
    println!("{}", v.into_iter().map(|(_, _, i)| i).join(" "));
}
