use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
    }
    let digits = n.count_ones() as usize;
    let mut nums = vec![];
    let mut i = 0usize;
    while nums.len() < digits {
        if n >> i & 1 == 1 {
            nums.push(1 << i);
        }
        i += 1;
    }
    for i in 0usize..1 << digits {
        let mut ans = 0usize;
        for j in 0..digits {
            if i >> j & 1 == 1 {
                ans += nums[j];
            }
        }
        println!("{}", ans);
    }
}
