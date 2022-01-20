use bitset_fixed::BitSet;

fn main() {
    proconio::input! {
        n: usize,
        t: [usize; n],
    }

    let tsum = t.iter().sum::<usize>() as usize;

    let mut dp = BitSet::new(tsum + 1);
    dp.set(0, true);

    for i in 0..n {
        dp |= &(&dp << t[i]);
    }

    println!("{}", ((tsum + 1) / 2..).find(|&x| dp[x]).unwrap());
}
