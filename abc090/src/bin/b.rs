use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!(
        "{}",
        (a..=b)
            .into_iter()
            .filter(|&i| get_digit(i, 1) == get_digit(i, 5) && get_digit(i, 2) == get_digit(i, 4))
            .count()
    );
}

// 下からi桁目の数字を返却する
fn get_digit(n: usize, i: usize) -> usize {
    n % 10usize.pow(i as u32) / 10usize.pow(i as u32 - 1)
}
