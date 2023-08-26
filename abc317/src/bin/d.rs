use proconio::input;

fn main() {
    input! {
        n:usize,
        xyz:[(usize,usize,usize);n],
    }
    let mut s = 0;
    for i in 0..n {
        s += xyz[i].2;
    }
    // これより大きい
    let need = s / 2;
    let mut win = 0;
    let mut lose = vec![];
    for (x, y, z) in xyz {
        if y < x {
            win += z;
        } else {
            lose.push(((y - x) / 2 + 1, z));
        }
    }
    if need < win {
        println!("0");
        return;
    }
    let mut dp = vec![!0; 2 + need - win];
    dp[0] = 0;
    let l = dp.len();
    for (ne, giseki) in lose {
        for i in (0..l).rev() {
            if dp[i] == !0 {
                continue;
            }
            dp[(i + giseki).min(l - 1)] = dp[(i + giseki).min(l - 1)].min(dp[i] + ne);
        }
    }
    println!("{}", dp[l - 1]);
}
