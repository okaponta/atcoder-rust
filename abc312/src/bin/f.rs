use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        tx:[(u8,usize);n],
    }
    let mut free_cans = vec![];
    let mut all_cans = vec![];
    let mut openers = vec![];
    for (t, x) in tx {
        if t == 0 {
            free_cans.push(x);
            all_cans.push((x, t));
        } else if t == 1 {
            all_cans.push((x, t));
        } else {
            openers.push(x);
        }
    }
    free_cans.resize(n, 0);
    all_cans.resize(n, (0, 0));
    free_cans.sort();
    free_cans.reverse();
    all_cans.sort();
    all_cans.reverse();
    openers.sort();
    openers.reverse();
    let mut s_free = vec![0];
    let mut s_all = vec![0];
    for i in 0..n {
        s_free.push(s_free[i] + free_cans[i]);
    }
    for i in 0..n {
        s_all.push(s_all[i] + all_cans[i].0);
    }

    let mut ans = s_free[m.min(free_cans.len())];
    let mut all_index = 0;
    let mut free_index = 0;
    let mut openers_num = 0;
    for x in openers {
        openers_num += 1;
        let mut opened = 0;
        while opened < x {
            if all_cans[all_index].1 == 1 {
                opened += 1;
            } else {
                free_index += 1;
            }
            all_index += 1;
            if m < all_index + openers_num {
                break;
            }
            let tmp = s_all[all_index.min(all_cans.len())]
                + s_free[(free_index + m - all_index - openers_num).min(free_cans.len())]
                - s_free[free_index.min(free_cans.len())];
            ans = ans.max(tmp);
        }
        if m < all_index + openers_num {
            break;
        }
    }
    println!("{}", ans);
}
