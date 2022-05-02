use proconio::input;

fn main() {
    input! {
        a:[i32;3],
    }
    let ans = a.iter().sum::<i32>() > 21;
    println!("{}", if ans { "bust" } else { "win" });
}
