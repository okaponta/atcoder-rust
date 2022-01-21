use proconio::input;

fn main() {
    input! {
       n:usize,
       a:[usize;n],
    }
    let mut ans = 1 << 30;
    // 2^nのbit全探索
    for i in 0..1 << n {
        let mut xor = 0;
        let mut or = 0;
        for j in 0..n {
            or = or | a[j];
            if i & (1 << j) > 0 {
                // フラグが立ってたらXOR
                xor = xor ^ or;
                or = 0;
            }
        }
        xor = xor ^ or;
        ans = ans.min(xor);
    }
    println!("{}", ans);
}
