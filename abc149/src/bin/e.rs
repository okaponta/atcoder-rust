use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.reverse();
    let s = ruiseki(&a);
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        let (count, _) = is_ok(med, &a, &s);
        if count < m {
            upper = med;
        } else {
            lower = med;
        }
    }
    let (count, mut ans) = is_ok(lower, &a, &s);
    if m < count {
        ans -= lower * (count - m);
    }
    println!("{}", ans);
}

fn is_ok(med: usize, a: &Vec<usize>, s: &Vec<usize>) -> (usize, usize) {
    let mut res = 0;
    let mut count = 0;
    for &ai in a {
        let pos = upper_bound_rev(a, med.saturating_sub(ai));
        count += pos;
        res += s[pos] + pos * ai;
    }
    (count, res)
}

// 逆順にソートされていること
// kを挿入するとしたらどのインデックスに入るかが返却される(大きい側)。
// [10,9,7,7,3]
// 11 -> 0
// 10 -> 1
// 9 -> 2
// 8 -> 2
// 7 -> 4
// 6 -> 4
fn upper_bound_rev(a: &Vec<usize>, k: usize) -> usize {
    if a[0] < k {
        return 0;
    }
    let mut lower = 0;
    let mut upper = a.len();
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if k <= a[med] {
            lower = med;
        } else {
            upper = med;
        }
    }
    upper
}

fn ruiseki(a: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
