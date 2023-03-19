use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:Usize1,
        ab:[(usize,usize);n],
        cd:[(usize,usize);m],
    }
    let mut lower = 0.0;
    let mut upper = 100.0;
    while upper - lower > 1e-10 {
        let med = (lower + upper) / 2.0;
        if is_over_k(&ab, &cd, med, k) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", upper);
}

fn is_over_k(ab: &Vec<(usize, usize)>, cd: &Vec<(usize, usize)>, med: f64, k: usize) -> bool {
    let mut sug = vec![];
    for &(a, b) in ab {
        sug.push((a + b) as f64 * med / 100.0 - a as f64);
    }
    sug.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut count = 0;
    for &(c, d) in cd {
        let su = (c + d) as f64 * med / 100.0 - c as f64;
        count += lower_bound(&sug, -su);
    }
    k < count
}

fn lower_bound(v: &Vec<f64>, t: f64) -> usize {
    let mut lower = 0;
    let mut upper = v.len();
    while upper != lower {
        let med = (lower + upper) / 2;
        if v[med] < t {
            lower = med + 1;
        } else {
            upper = med;
        }
    }
    lower
}
