use proconio::{input, marker::Chars};

fn main() {
    input! {
        k:usize,
        mut s:Chars,
        mut t:Chars,
    }
    let mut ss = vec![0; 10];
    let mut tt = vec![0; 10];
    let mut remain = vec![k; 10];
    s.remove(4);
    t.remove(4);
    for c in s {
        ss[c.to_digit(10).unwrap() as usize] += 1;
        remain[c.to_digit(10).unwrap() as usize] -= 1;
    }
    for c in t {
        tt[c.to_digit(10).unwrap() as usize] += 1;
        remain[c.to_digit(10).unwrap() as usize] -= 1;
    }

    let mut ans = 0.0;
    for s_last in 1..10 {
        if remain[s_last] == 0 {
            continue;
        }

        let p = remain[s_last] as f64 / (k * 9 - 8) as f64;
        remain[s_last] -= 1;
        ss[s_last] += 1;

        for t_last in 1..10 {
            if remain[t_last] == 0 {
                continue;
            }
            let pp = remain[t_last] as f64 / (k * 9 - 9) as f64;
            tt[t_last] += 1;
            if is_s_win(&ss, &tt) {
                ans += p * pp;
            }
            tt[t_last] -= 1;
        }

        ss[s_last] -= 1;
        remain[s_last] += 1;
    }
    println!("{}", ans);
}

fn is_s_win(s: &Vec<usize>, t: &Vec<usize>) -> bool {
    point(s) > point(t)
}

fn point(v: &Vec<usize>) -> usize {
    let mut res = 0;
    for i in 0..10 {
        res += i * 10_usize.pow(v[i] as u32);
    }
    res
}
