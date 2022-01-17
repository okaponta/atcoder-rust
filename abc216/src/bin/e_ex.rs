use proconio::input;

fn ride_times(v: &Vec<u64>, m: u64) -> u64 {
    v.iter().map(|x| (x + 1u64).saturating_sub(m)).sum()
}

fn main() {
    input! {
        n:usize,
        k:u64,
        anums:[u64;n],
    }
    let mut left = 0;
    let mut right = 2000000001;

    while right - left > 1 {
        let mid = (left + right) / 2;
        if ride_times(&anums, mid) <= k {
            right = mid;
        } else {
            left = mid;
        }
    }
    let t = ride_times(&anums, right);
    let ans: u64 = anums
        .iter()
        .map(|x| (x * (x + 1) / 2).saturating_sub(right * (right - 1) / 2))
        .sum::<u64>()
        + (right - 1) * (k - t);
    println!("{}", ans);
}
