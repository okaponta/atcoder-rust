use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        cp:[(Usize1,usize);n],
        q:usize,
        ab:[(Usize1,usize);q],
    }
    let mut one = vec![0; n];
    let mut two = vec![0; n];
    for i in 0..n {
        if cp[i].0 == 0 {
            one[i] += cp[i].1;
        } else {
            two[i] += cp[i].1;
        }
    }
    let s1 = ruiseki(&one);
    let s2 = ruiseki(&two);
    for (a, b) in ab {
        println!("{} {}", s1[b] - s1[a], s2[b] - s2[a]);
    }
}

// 累積和
fn ruiseki(a: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
