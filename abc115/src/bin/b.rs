use proconio::input;

fn main() {
    input! {
        n:usize,
        mut p:[usize;n],
    }
    p.sort();
    p[n - 1] = p[n - 1] / 2;
    println!("{}", p.into_iter().sum::<usize>());
}
