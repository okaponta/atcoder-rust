use proconio::input;

fn main() {
    input! {
            s: String,
    }
    let ans: String = s
        .chars()
        .rev()
        .map(|c| match c {
            '6' => '9',
            '9' => '6',
            c => c,
        })
        .collect();
    println!("{}", ans)
}
