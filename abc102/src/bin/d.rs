use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut s = vec![0];
    for i in 0..n {
        s.push(s[s.len() - 1] + a[i]);
    }
    let mut ans = 1 << 60;
    for i in 1..n {
        ans = ans.min(f(i, n, &s));
    }
    println!("{}", ans);
}

fn f(med: usize, n: usize, s: &Vec<usize>) -> usize {
    let a = g(0, med, s);
    let c = g(med, n, s);
    let mut v = vec![];
    v.push(s[a] - s[0]);
    v.push(s[med] - s[a]);
    v.push(s[c] - s[med]);
    v.push(s[n] - s[c]);
    v.sort();
    v[3] - v[0]
}

fn g(l: usize, r: usize, s: &Vec<usize>) -> usize {
    let mut lower = l;
    let mut upper = r;
    while 2 < upper - lower {
        let m1 = (lower * 2 + upper) / 3;
        let m2 = (lower + upper * 2) / 3;
        let h1 = h(l, r, m1, s);
        let h2 = h(l, r, m2, s);
        if h1 <= h2 {
            upper = m2;
        } else {
            lower = m1;
        }
    }
    (lower..upper)
        .into_iter()
        .map(|i| (h(l, r, i, s), i))
        .min()
        .unwrap()
        .1
}

fn h(l: usize, r: usize, mid: usize, s: &Vec<usize>) -> usize {
    let s1 = s[mid] - s[l];
    let s2 = s[r] - s[mid];
    abs(s1, s2)
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
