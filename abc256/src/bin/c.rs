use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        h:[usize;3],
        w:[usize;3],
    }
    let mut ans = 0;
    for (aa, ab, ba, bb) in iproduct!(1..30, 1..30, 1..30, 1..30) {
        let ac = h[0].saturating_sub(aa + ab);
        let bc = h[1].saturating_sub(ba + bb);
        let ca = w[0].saturating_sub(aa + ba);
        let cb = w[1].saturating_sub(ab + bb);
        let cc1 = w[2].saturating_sub(ac + bc);
        let cc2 = h[2].saturating_sub(ca + cb);
        if ac == 0 || bc == 0 || ca == 0 || cb == 0 || cc1 != cc2 || cc1 == 0 {
            continue;
        }
        ans += 1;
    }
    println!("{}", ans);
}
