use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [String; h],
        b: [String; h],
    }

    let mut ans = "No";

    // s回の縦方向のシフトとt回の横方向のシフトをすべて試す
    for s in 0..h {
        for t in 0..w {
            let mut match_flag = true;
            // AをシフトしてBと一致するかどうか調べる
            for i in 0..h {
                for j in 0..w {
                    let x = (i + s) % h;
                    let y = (j + t) % w;
                    if a[x].chars().nth(y).unwrap() != b[i].chars().nth(j).unwrap() {
                        match_flag = false;
                    }
                }
            }
            if match_flag {
                ans = "Yes";
                break;
            }
        }
    }

    println!("{}", ans);
}
