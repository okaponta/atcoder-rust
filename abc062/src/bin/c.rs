use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
    }
    let mut ans = h * w;
    // 横
    let a = vec![h / 3, (h / 3) + 1];
    for a in a {
        let sa = a * w;
        // 横
        let b = vec![(h - a) / 2, ((h - a) / 2) + 1];
        for b in b {
            let sb = b * w;
            let sc = (h - a - b) * w;
            ans = ans.min(calc(sa, sb, sc));
        }
        // 縦
        let b = vec![w / 2, (w / 2) + 1];
        for b in b {
            let sb = b * (h - a);
            let sc = (w - b) * (h - a);
            ans = ans.min(calc(sa, sb, sc));
        }
    }
    // 縦
    let a = vec![w / 3, (w / 3) + 1];
    for a in a {
        let sa = h * a;
        // 横
        let b = vec![h / 2, (h / 2) + 1];
        for b in b {
            let sb = b * (w - a);
            let sc = (h - b) * (w - a);
            ans = ans.min(calc(sa, sb, sc));
        }
        // 縦
        let b = vec![(w - a) / 2, (w - a) / 2 + 1];
        for b in b {
            let sb = b * h;
            let sc = (w - a - b) * h;
            ans = ans.min(calc(sa, sb, sc));
        }
    }
    println!("{}", ans);
}

fn calc(sa: usize, sb: usize, sc: usize) -> usize {
    sa.max(sb).max(sc) - sa.min(sb).min(sc)
}
