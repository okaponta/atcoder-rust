use proconio::input;

fn main() {
    input! {
       n:usize,m:usize,
       b:[[i32;m];n],
    }
    for i in 0..n {
        for j in 0..m {
            if i == 0 {
                if j != 0 {
                    if b[i][j] != b[i][j - 1] + 1 {
                        println!("No");
                        return;
                    }
                    if b[i][j] % 7 == 1 {
                        println!("No");
                        return;
                    }
                }
            } else {
                if b[i][j] != b[i - 1][j] + 7 {
                    println!("No");
                    return;
                }
            }
        }
    }
    println!("Yes");
}
