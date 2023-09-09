use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        l:[usize;n],
    }
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, &l, m) {
            upper = med;
        } else {
            lower = med;
        }
    }
    println!("{}", upper);
}

fn is_ok(row: usize, l: &Vec<usize>, m: usize) -> bool {
    let mut ans = 0;
    let mut cur = row;
    for &l in l {
        if row < l {
            return false;
        }
        cur += l + 1;
        if row < cur {
            // 改行
            ans += 1;
            cur = l;
        }
        //cur += 1;
    }
    ans <= m
}
