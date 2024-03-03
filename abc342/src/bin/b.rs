use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        p:[Usize1;n],
        q:usize,
        ab:[(Usize1,Usize1);q],
    }
    let mut ord = vec![0; n];
    for i in 0..n {
        ord[p[i]] = i;
    }
    for (a, b) in ab {
        println!("{}", if ord[a] < ord[b] { a + 1 } else { b + 1 });
    }
}
