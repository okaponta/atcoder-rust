use proconio::input;

fn main() {
    input! {
        n:usize,
        d:usize,
        a:[usize;n],
    }
    let mut ans = 1usize << d;
    for i in 1_usize..1 << n {
        let mut bit = usize::max_value() - ((1 << d) - 1);
        for j in 0..n {
            if i >> j & 1 == 1 {
                bit |= a[j];
            }
        }
        let s = 1 << bit.count_zeros();
        if i.count_ones() & 1 == 1 {
            ans -= s;
        } else {
            ans += s;
        }
    }
    println!("{}", ans);
}
