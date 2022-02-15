use proconio::input;

fn main() {
    input! {
        n:usize,l:usize,
        k:usize,
        a:[usize;n],
    }
    let mut len = vec![0; n + 1];
    for i in 0..n {
        if i == 0 {
            len[0] = a[i];
        } else {
            len[i] = a[i] - a[i - 1];
        }
    }
    len[n] = l - a[n - 1];

    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(&len, med, k) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(len: &[usize], med: usize, k: usize) -> bool {
    let mut tmp = 0;
    let mut count = 0;
    for i in len {
        tmp += i;
        if tmp >= med {
            tmp = 0;
            count += 1;
        }
    }
    count > k
}
