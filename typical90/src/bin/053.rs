use proconio::input_interactive;
use std::collections::HashMap;

fn main() {
    input_interactive! {
        t: usize,
    }
    for _ in 0..t {
        test();
    }
}

fn test() {
    input_interactive! {
        n: usize,
    }
    let fibo = build_fibo(n + 1);

    let mut map = HashMap::new();
    // 左端をlで管理し、黄金分割探索で範囲を狭めていく
    let mut l = 0;
    for v in fibo.windows(3).rev() {
        let l0 = l + v[0];
        let r0 = l + v[1];
        if f(r0, n, &mut map) > f(l0, n, &mut map) {
            l = l0;
        }
    }
    println!("! {}", f(l + 1, n, &mut map));
}

// フィボナッチ数列を n 以上の値が現れるまで組み立てる
fn build_fibo(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1];
    }
    let mut v = vec![1, 2];
    while v[v.len() - 1] < n {
        let x = v[v.len() - 2] + v[v.len() - 1];
        v.push(x);
    }
    v
}

fn f(i: usize, n: usize, map: &mut HashMap<usize, usize>) -> usize {
    let i = i.min(n);
    if let Some(&x) = map.get(&i) {
        return x;
    }

    println!("? {}", i);
    input_interactive! {
        x: usize,
    }
    map.insert(i, x);
    x
}
