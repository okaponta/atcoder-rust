use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[u8;w];h],
    }
    for a in a {
        println!(
            "{}",
            a.into_iter()
                .map(|i| if i == 0 { '.' } else { (b'A' + i - 1) as char })
                .collect::<String>()
        )
    }
}
