use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        x:usize,
        y:usize,
        pt:[(usize,usize);n-1],
        q:usize,
        qi:[usize;q],
    }
    let mut bus = vec![0; 840];
    for i in 0..840 {
        let mut cur = i;
        for &(p, t) in &pt {
            cur = ((cur + p - 1) / p) * p + t;
        }
        bus[i] = cur;
    }
    for qi in qi {
        let mut ans = ((qi + x) / 840) * 840;
        ans += bus[(qi + x) % 840];
        ans += y;
        println!("{}", ans);
    }
}
