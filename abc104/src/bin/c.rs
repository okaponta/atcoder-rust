use proconio::input;

fn main() {
    input! {
        d:usize,
        g:usize,
        pc:[(usize,usize);d],
    }
    let mut comp = vec![0; d];
    for i in 0..d {
        comp[i] = pc[i].0 * ((i + 1) * 100) + pc[i].1;
    }
    let mut ans = 1usize << 60;
    for i in 0..1 << d {
        let mut tmp = 0;
        let mut score = 0;
        for j in 0..d {
            if i >> j & 1 == 1 {
                tmp += pc[j].0;
                score += comp[j];
            }
        }
        if g <= score {
            ans = ans.min(tmp);
            continue;
        }
        let mut max = 0;
        for j in (0..d).rev() {
            if i >> j & 1 != 1 {
                max = j;
                break;
            }
        }
        if pc[max].0 <= (g - score + max * 100 + 99) / ((max + 1) * 100) {
            continue;
        }
        ans = ans.min(tmp + (g - score + max * 100 + 99) / ((max + 1) * 100));
    }
    println!("{}", ans);
}
