use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut ha:usize,
        mut wa:usize,
        mut a:[Chars;ha],
        mut hb:usize,
        mut wb:usize,
        mut b:[Chars;hb],
        hx:usize,
        wx:usize,
        x:[Chars;hx],
    }
    while (0..wa).into_iter().all(|j| a[0][j] == '.') {
        a.remove(0);
        ha -= 1;
    }
    while (0..wa).into_iter().rev().all(|j| a[ha - 1][j] == '.') {
        a.remove(ha - 1);
        ha -= 1;
    }
    while (0..ha).into_iter().all(|i| a[i][0] == '.') {
        for j in 0..ha {
            a[j].remove(0);
        }
        wa -= 1;
    }
    while (0..ha).into_iter().rev().all(|i| a[i][wa - 1] == '.') {
        for j in 0..ha {
            a[j].remove(wa - 1);
        }
        wa -= 1;
    }
    while (0..wb).into_iter().all(|j| b[0][j] == '.') {
        b.remove(0);
        hb -= 1;
    }
    while (0..wb).into_iter().rev().all(|j| b[hb - 1][j] == '.') {
        b.remove(hb - 1);
        hb -= 1;
    }
    while (0..hb).into_iter().all(|i| b[i][0] == '.') {
        for j in 0..hb {
            b[j].remove(0);
        }
        wb -= 1;
    }
    while (0..hb).into_iter().rev().all(|i| b[i][wb - 1] == '.') {
        for j in 0..hb {
            b[j].remove(wb - 1);
        }
        wb -= 1;
    }
    if hx < ha || hx < hb || wx < wa || wx < wb {
        println!("No");
        return;
    }
    // for i in 0..ha {
    //     println!("{:?}", a[i]);
    // }
    // for i in 0..hb {
    //     println!("{:?}", b[i]);
    // }
    // aの開始
    for i in 0..=hx - ha {
        for j in 0..=wx - wa {
            // bの開始
            for k in 0..=hx - hb {
                for l in 0..=wx - wb {
                    let mut ans = vec![vec!['.'; wx]; hx];
                    for m in 0..ha {
                        for n in 0..wa {
                            if a[m][n] == '#' {
                                ans[i + m][j + n] = '#';
                            }
                        }
                    }
                    for m in 0..hb {
                        for n in 0..wb {
                            if b[m][n] == '#' {
                                ans[k + m][l + n] = '#';
                            }
                        }
                    }
                    let mut is_ans = true;
                    for m in 0..hx {
                        for n in 0..wx {
                            if ans[m][n] != x[m][n] {
                                is_ans = false;
                            }
                        }
                    }
                    // for i in 0..hx {
                    //     println!("{:?}", ans[i]);
                    // }
                    if is_ans {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
