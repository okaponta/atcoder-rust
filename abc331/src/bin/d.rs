use itertools::iproduct;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        p:[Chars;n],
        abcd:[(usize,usize,usize,usize);q],
    }
    let mut a = vec![vec![0usize; n]; n];
    for (i, j) in iproduct!(0..n, 0..n) {
        if p[i][j] == 'B' {
            a[i][j] += 1;
        }
    }
    let mut s = TwoDSum::new(n, n, &a);
    for (a, b, c, d) in abcd {
        let ans = g(c + 1, d + 1, n, &mut s) + g(a, b, n, &mut s)
            - g(a, d + 1, n, &mut s)
            - g(c + 1, b, n, &mut s);
        println!("{}", ans);
    }
}

fn g(a: usize, b: usize, n: usize, s: &mut TwoDSum) -> usize {
    let e = a / n;
    let f = b / n;
    s.get_o(n, n) * e * f + s.get_o(a % n, n) * f + s.get_o(n, b % n) * e + s.get_o(a % n, b % n)
}

pub struct TwoDSum {
    s: Vec<Vec<usize>>,
}

impl TwoDSum {
    pub fn new(h: usize, w: usize, a: &Vec<Vec<usize>>) -> Self {
        let mut s = vec![vec![0; w + 1]; h + 1];
        for i in 1..=h {
            for j in 1..=w {
                s[i][j] = a[i - 1][j - 1] + s[i - 1][j] + s[i][j - 1] - s[i - 1][j - 1];
            }
        }
        TwoDSum { s }
    }

    // [h1,h2) * [w1,w2)の区間和を返却する
    pub fn get(&mut self, h1: usize, w1: usize, h2: usize, w2: usize) -> usize {
        self.s[h2][w2] + self.s[h1][w1] - self.s[h1][w2] - self.s[h2][w1]
    }

    // [0,h) * [0,w0)の区間和を返却する
    pub fn get_o(&mut self, h: usize, w: usize) -> usize {
        self.s[h][w] + self.s[0][0] - self.s[0][w] - self.s[h][0]
    }
}
