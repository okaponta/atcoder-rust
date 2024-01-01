use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        s:[String;h],
    }
    let header = (0..w + 2).into_iter().map(|_| '#').collect::<String>();
    println!("{}", header);
    for s in s {
        println!("#{}#", s);
    }
    println!("{}", header);
}
