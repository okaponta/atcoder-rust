use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut l:[usize;n],
    }
    l.sort();
    l.reverse();
    println!("{}", (0..k).into_iter().fold(0, |s, i| s + l[i]));
}
