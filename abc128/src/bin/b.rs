use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        sp: [(String,usize);n],
    }
    let mut spi = vec![];
    for (i, (s, p)) in sp.iter().enumerate() {
        spi.push((s, p, i + 1));
    }
    spi.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));
    for (_, _, i) in spi {
        println!("{}", i);
    }
}
