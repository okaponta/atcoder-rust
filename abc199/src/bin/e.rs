use proconio::input;

fn main() {
    input! {
       n:usize,
       m:usize,
       xyz:[(usize,usize,u32);m],
    }
    let mut yz = vec![vec![]; n];
    for (x, y, z) in xyz {
        yz[x].push((y, z));
    }
    let mut dp = vec![0usize; 1 << n];
    dp[0] = 1;
    for i in 0usize..(1 << n) - 1 {
        if yz[i.count_ones() as usize]
            .iter()
            .any(|&(y, z)| z < (i & ((1 << y) - 1)).count_ones())
        {
            continue;
        }
        for j in 0..n {
            if i >> j & 1 == 1 {
                continue;
            }
            dp[i | 1 << j] += dp[i];
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
