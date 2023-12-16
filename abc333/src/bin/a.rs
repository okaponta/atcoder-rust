use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:Usize1,
    }
    let ans = vec![
        1usize, 22, 333, 4444, 55555, 666666, 7777777, 88888888, 999999999,
    ];
    println!("{}", ans[n]);
}
