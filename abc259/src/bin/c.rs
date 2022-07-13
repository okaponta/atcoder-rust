use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let mut sv = vec![];
    let mut prev = s[0];
    let mut count = 0;
    for c in s {
        if c == prev {
            count += 1;
        } else {
            sv.push((prev, count));
            prev = c;
            count = 1;
        }
    }
    sv.push((prev, count));
    let mut pv = vec![];
    prev = t[0];
    count = 0;
    for c in t {
        if c == prev {
            count += 1;
        } else {
            pv.push((prev, count));
            prev = c;
            count = 1;
        }
    }
    pv.push((prev, count));

    if sv.len() != pv.len() {
        println!("No");
        return;
    }
    for i in 0..sv.len() {
        let si = sv[i];
        let pi = pv[i];
        if si.0 != pi.0 {
            println!("No");
            return;
        }
        if si.1 == 1 {
            if pi.1 != 1 {
                println!("No");
                return;
            }
        }
        if 1 < si.1 {
            if pi.1 < si.1 {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
