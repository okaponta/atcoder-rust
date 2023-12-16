use proconio::{input, marker::Chars};

fn main() {
    input! {
        r:Chars,
        n:u128,
    }
    let (pt, qt) = cv_to_frac(r);
    let (p, q) = find_irreducible_frac(pt, qt, n);
    println!("{} {}", p, q);
}

fn cv_to_frac(mut r: Vec<char>) -> (u128, u128) {
    r.resize(21, '0');
    let mut pt = r[2..].into_iter().collect::<String>().parse().unwrap();
    let mut qt = 10e18 as u128;
    let g = gcd(pt, qt);
    pt /= g;
    qt /= g;
    (pt, qt)
}

fn gcd(a: u128, b: u128) -> u128 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

// pt/qtに近い、n以下で表現される規約分数を返却する
fn find_irreducible_frac(pt: u128, qt: u128, n: u128) -> (u128, u128) {
    if pt <= n && qt <= n {
        return (pt, qt);
    }

    let mut pl = 0;
    let mut ql = 1;
    let mut pr = 1;
    let mut qr = 0;

    while ql <= n && qr <= n {
        let (npl, nql, npr, nqr) = stern_brocot(pt, qt, pl, ql, pr, qr, n);
        if npl <= n && nql <= n && npr <= n && nqr <= n {
            (pl, ql, pr, qr) = (npl, nql, npr, nqr);
        } else {
            break;
        }
    }
    let d1 = pt * ql * qr - pl * qt * qr;
    let d2 = pr * ql * qt - pt * ql * qr;
    if d1 <= d2 {
        return (pl, ql);
    } else {
        return (pr, qr);
    }
}

// stern brocot木でひとつ下の階層の範囲を返却する
fn stern_brocot(
    pt: u128,
    qt: u128,
    pl: u128,
    ql: u128,
    pr: u128,
    qr: u128,
    n: u128,
) -> (u128, u128, u128, u128) {
    let pm = pl + pr;
    let qm = ql + qr;
    // 1/1や1/2の付近は分母が大きくならないのでまとめて計算して効率化する
    // nをこえないようにminの値を設定するが、0だと進展が無いので最小値は1にする
    if pt * qm < qt * pm {
        let x = ((pr * qt - pt * qr) / (pt * ql - pl * qt))
            .min((n - qr) / ql)
            .max(1);
        return (pl, ql, pl * x + pr, ql * x + qr);
    } else {
        let x = ((pt * ql - pl * qt) / (pr * qt - pt * qr))
            .min((n - ql) / qr)
            .max(1);
        return (pl + pr * x, ql + qr * x, pr, qr);
    }
}
