use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut koma = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                koma.push((i as i32, j as i32));
            }
        }
    }
    let ans = (koma[0].0 - koma[1].0).abs() + (koma[0].1 - koma[1].1).abs();
    println!("{}", ans);
}
