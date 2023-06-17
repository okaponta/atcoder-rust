use std::collections::BTreeSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        k:usize,
        q:usize,
        xy:[(Usize1,usize);q],
    }
    let mut a = vec![0; n];
    let mut is_ans = vec![false; n];
    let mut seta = BTreeSet::new();
    let mut setb = BTreeSet::new();
    for i in 0..n {
        if i < k {
            seta.insert((0, i));
            is_ans[i] = true;
        } else {
            setb.insert((0, i));
        }
    }
    let mut ans = 0;
    for (x, y) in xy {
        let before = a[x];
        if is_ans[x] {
            seta.remove(&(before, x));
            seta.insert((y, x));
            ans -= before;
            ans += y;
        } else {
            setb.remove(&(before, x));
            setb.insert((y, x));
        }
        a[x] = y;
        if setb.len() != 0 {
            let mina = *seta.iter().next().unwrap();
            let maxb = *setb.iter().last().unwrap();
            if mina < maxb {
                seta.remove(&mina);
                setb.remove(&maxb);
                ans -= mina.0;
                seta.insert(maxb);
                ans += maxb.0;
                setb.insert(mina);
                is_ans[mina.1] = false;
                is_ans[maxb.1] = true;
            }
        }
        println!("{}", ans);
    }
}
