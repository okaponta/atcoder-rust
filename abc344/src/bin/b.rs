use proconio::{fastout, input};

#[fastout]
fn main() {
    let mut a = vec![];
    loop {
        input! {x:usize}
        a.push(x);
        if x == 0 {
            break;
        }
    }
    a.reverse();
    for a in a {
        println!("{}", a);
    }
}
