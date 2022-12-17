use proconio::input;

fn main() {
    input! {
        k:usize,
    }
    let mut ans = vec![];
    for i in 0..k {
        ans.push((b'A' + i as u8) as char);
    }
    println!("{}", ans.into_iter().collect::<String>());
}
