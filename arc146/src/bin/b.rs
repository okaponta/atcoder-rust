use proconio::input;

fn main() {
    input! {
        n:usize,
        mut m:usize,
        k:usize,
        mut a:[usize;n],
    }
    a.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    let mut flag = false;
    let mut target = 1 << 32;

    while 0 < target {
        let (ok, sub) = sub(&a, k, target, m);
        if ok {
            m -= sub;
            ans += target;
            if a[k - 1] < target {
                // 順序決定
                flag = true;
            }
            for i in 0..n {
                if target <= a[i] {
                    a[i] -= target;
                } else {
                    a[i] = 0;
                }
            }
        } else {
            for i in 0..n {
                if target <= a[i] {
                    a[i] -= target;
                }
            }
        }
        target >>= 1;
        if !flag {
            a.sort_by(|a, b| b.cmp(a));
        }
    }
    println!("{}", ans);
}

fn sub(a: &Vec<usize>, k: usize, n: usize, m: usize) -> (bool, usize) {
    let mut res = 0;
    for i in 0..k {
        if n <= a[i] {
            continue;
        }
        res += n - a[i];
        if res > m {
            return (false, 0);
        }
    }
    return (true, res);
}
