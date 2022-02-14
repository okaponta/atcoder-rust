use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,
        c:Chars,
    }
    let red = c.clone().into_iter().filter(|&c| c == 'R').count();
    let ok = (0..red).into_iter().filter(|&i| c[i] == 'R').count();
    println!("{}", red - ok);
}
