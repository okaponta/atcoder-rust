use proconio::input;

fn main() {
    input! {
        n:usize,
        abcd:[(usize,usize,usize,usize);n],
    }
    let mut sheet = vec![vec![false; 101]; 101];
    for (a, b, c, d) in abcd {
        for i in a..b {
            for j in c..d {
                sheet[i][j] = true;
            }
        }
    }
    let mut ans = 0;
    for i in 0..101 {
        for j in 0..101 {
            if sheet[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
