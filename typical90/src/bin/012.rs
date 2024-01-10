use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        q:usize,
    }
    let mut used = vec![vec![false; w]; h];
    let mut uf = UnionFind::new(h * w);
    for _ in 0..q {
        input! {q: u8}
        if q == 1 {
            input! {r: Usize1, c:Usize1}
            used[r][c] = true;
            for (dr, dc) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if h <= nr || w <= nc {
                    continue;
                }
                if used[nr][nc] {
                    uf.union(r * w + c, nr * w + nc);
                }
            }
        } else {
            input! {ra: Usize1, ca:Usize1, rb: Usize1, cb: Usize1}
            println!(
                "{}",
                if used[ra][ca] && used[rb][cb] && uf.equiv(ra * w + ca, rb * w + cb) {
                    "Yes"
                } else {
                    "No"
                }
            )
        }
    }
}
