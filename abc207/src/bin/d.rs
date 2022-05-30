use proconio::input;

fn main() {
    input! {
       n:usize,
       ab:[(i32,i32);n],
       cd:[(i32,i32);n],
    }
    if n == 1 {
        println!("Yes");
        return;
    }
    if n == 2 {
        let is_same = dist(ab[0], ab[1]) == dist(cd[0], cd[1]);
        println!("{}", if is_same { "Yes" } else { "No" });
        return;
    }
    // 点0,1と同じ長さのものがあるかを探索
    let d = dist(ab[0], ab[1]);
    let mut base_pair = vec![];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if dist(cd[i], cd[j]) == d {
                base_pair.push((i, j));
            }
        }
    }

    // 外積と距離が一致するものを探索
    for (i, j) in base_pair {
        // cdの中で使用された点
        let mut used = vec![false; n];
        used[i] = true;
        used[j] = true;

        // kはもとのabの点たち
        for k in 2..n {
            // 外積
            let outer = outer_product_p(ab[0], ab[1], ab[k]);
            //距離
            let dd = dist(ab[0], ab[k]);
            // 上記に対応するcdの点があるかをチェック
            let mut update_flag = false;
            for l in 0..n {
                if used[l] {
                    continue;
                }
                if outer_product_p(cd[i], cd[j], cd[l]) == outer && dist(cd[i], cd[l]) == dd {
                    // OK
                    used[l] = true;
                    update_flag = true;
                    break;
                }
            }
            if !update_flag {
                break;
            }
            if k == n - 1 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1) * (p1.1 - p2.1)
}

fn outer_product_p(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> i32 {
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let c = p3.0 - p1.0;
    let d = p3.1 - p1.1;
    a * d - b * c
}
