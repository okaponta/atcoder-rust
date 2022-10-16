use std::cmp::Ordering::{Equal, Greater, Less};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Index,
};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
        t:Chars,
    }
    let mut x = vec![];
    for i in 0..2 * n {
        x.push(s[i % n]);
    }
    for _ in 0..n {
        x.push('a');
    }
    for i in 0..2 * n {
        x.push(t[i % n]);
    }
    for _ in 0..n {
        x.push('z');
    }
    let buf = x.iter().collect::<String>();
    let sa: SuffixArray<_> = buf.into();
    let sa: Vec<_> = sa.into();
    let mut ans = 0;
    let mut count = n;
    for i in 0..6 * n {
        if sa[i] < n {
            ans += count;
        }
        if 3 * n <= sa[i] && sa[i] < 4 * n {
            count -= 1;
        }
    }
    println!("{}", ans);
}

// https://rsk0315.github.io/library-rs/nekolib/seq/struct.SuffixArray.html
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuffixArray<T: Ord> {
    buf: Vec<T>,
    sa: Vec<usize>,
}

impl<T: Ord> From<Vec<T>> for SuffixArray<T> {
    fn from(buf: Vec<T>) -> Self {
        let buf_usize = hash(&buf);
        let sa = sa_is(&buf_usize);
        Self { buf, sa }
    }
}

impl From<String> for SuffixArray<char> {
    fn from(buf: String) -> Self {
        let buf: Vec<_> = buf.as_str().chars().collect();
        let buf_usize = hash_chars(&buf);
        let sa = sa_is(&buf_usize);
        Self { buf, sa }
    }
}

/// 座標圧縮をする。
///
/// `buf` の末尾に辞書順最小の文字 `$` を付加した列を、座標圧縮して返す。
fn hash<T: Ord>(buf: &[T]) -> Vec<usize> {
    let enc: BTreeSet<_> = buf.iter().collect();
    let enc: BTreeMap<_, _> = enc.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
    buf.iter()
        .map(|x| enc[x] + 1)
        .chain(std::iter::once(0)) // for '$'
        .collect()
}

/// 座標圧縮をする。
///
/// `buf` の末尾に辞書順最小の文字 `$` を付加した列を、座標圧縮して返す。`char`
/// の列を受け取り、バケットソートの要領で行う。
fn hash_chars(buf: &[char]) -> Vec<usize> {
    let max = match buf.iter().max() {
        Some(&c) => c as usize,
        None => return vec![0], // "$"
    };
    let enc = {
        let mut enc = vec![0; max + 1];
        for &c in buf {
            enc[c as usize] = 1;
        }
        for i in 1..=max {
            enc[i] += enc[i - 1];
        }
        enc
    };
    buf.iter()
        .map(|&x| enc[x as usize])
        .chain(std::iter::once(0)) // for '$'
        .collect()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LsType {
    LType,
    SType(bool), // true iff leftmost S-type
}
use LsType::{LType, SType};

/// 出現回数を求める。
///
/// `res[x]` が `buf` における `x` の出現回数となる配列 `res` を返す。
///
/// # Requirements
/// `buf` の要素は `0..buf.len()` に含まれる。
fn count_freq(buf: &[usize]) -> Vec<usize> {
    let mut res = vec![0; buf.len()];
    buf.iter().for_each(|&x| res[x] += 1);
    res
}

/// 逆順列を返す。
///
/// `res[x]` が `buf` における `x` の出現位置となる配列 `res` を返す。
///
/// # Requirements
/// `buf` の要素は `0..buf.len()` に含まれ、かつ distinct である。
fn inv_perm(buf: &[usize]) -> Vec<usize> {
    let mut res = vec![0; buf.len()];
    buf.iter().enumerate().for_each(|(i, &x)| res[x] = i);
    res
}

/// LS type を求める。
///
/// `res[i]` が `buf[i]` の LS type である配列 `res` を返す。
fn ls_classify(buf: &[usize]) -> Vec<LsType> {
    let mut res = vec![SType(false); buf.len()];
    for i in (0..buf.len() - 1).rev() {
        res[i] = match buf[i].cmp(&buf[i + 1]) {
            Less => SType(false),
            Equal => res[i + 1],
            Greater => LType,
        };
    }
    for i in 1..buf.len() {
        if let (LType, SType(_)) = (res[i - 1], res[i]) {
            res[i] = SType(true);
        }
    }
    res
}

/// 各バケットの開始位置を求める。
///
/// # Input
/// `count[i]` が `i` 番目のバケットのサイズである配列 `count`。
fn bucket_head(count: &[usize]) -> Vec<usize> {
    let n = count.len();
    let mut head: Vec<_> = std::iter::once(&0)
        .chain(&count[..n - 1])
        .cloned()
        .collect();
    for i in 1..n {
        head[i] += head[i - 1];
    }
    head
}

/// 各バケットの終端位置を求める。
///
/// # Input
/// `count[i]` が `i` 番目のバケットのサイズである配列 `count`。
fn bucket_tail(count: &[usize]) -> Vec<usize> {
    let mut tail = count.to_vec();
    for i in 1..count.len() {
        tail[i] += tail[i - 1];
    }
    tail
}

/// すでに判明している SA と LS type から、SA の残りの要素を求める。
fn induce(buf: &[usize], sa: &mut [Option<usize>], count: &[usize], ls: &[LsType]) {
    let mut head = bucket_head(count);
    for i in 0..sa.len() {
        if let Some(j) = sa[i] {
            if j > 0 && ls[j - 1] == LType {
                sa[head[buf[j - 1]]] = Some(j - 1);
                head[buf[j - 1]] += 1;
            }
        }
    }
    let mut tail = bucket_tail(count);
    for i in (1..count.len()).rev() {
        if let Some(j) = sa[i] {
            if j > 0 && ls[j - 1] != LType {
                tail[buf[j - 1]] -= 1;
                sa[tail[buf[j - 1]]] = Some(j - 1);
            }
        }
    }
}

/// 各 LMS block を文字とする文字列を作る。
///
/// 新たな文字も辞書順に番号づけられる。
///
/// # Examples
/// `[CCCG][AT][GTC][AT][GTC][AGGA][$]` → `3242410`
fn reduce(buf: &[usize], lms: &[usize], ls: &[LsType]) -> Vec<usize> {
    if lms.len() <= 1 {
        return vec![0; lms.len()];
    }

    let e = |(i0, i1)| {
        if (ls[i0], ls[i1]) == (SType(true), SType(true)) {
            Some(true)
        } else if ls[i0] != ls[i1] || buf[i0] != buf[i1] {
            Some(false)
        } else {
            None
        }
    };

    let mut map = vec![0; buf.len()]; // map[lms[0]] = 0
    map[lms[1]] = 1;
    let mut x = 1;
    for i in 2..lms.len() {
        let equiv = buf[lms[i]] == buf[lms[i - 1]]
            && (lms[i] + 1..).zip(lms[i - 1] + 1..).find_map(e).unwrap();
        if !equiv {
            x += 1;
        }
        map[lms[i]] = x;
    }

    (0..buf.len())
        .filter_map(|i| match ls[i] {
            SType(true) => Some(map[i]),
            _ => None,
        })
        .collect()
}

/// SA-IS により接尾辞配列を求める。
fn sa_is(buf: &[usize]) -> Vec<usize> {
    let len = buf.len();
    let count = count_freq(buf);
    if count.iter().all(|&x| x == 1) {
        return inv_perm(buf);
    }

    let ls = ls_classify(buf);
    let mut sa = vec![None; len];
    let mut tail = bucket_tail(&count);
    for i in (1..len).rev().filter(|&i| ls[i] == SType(true)) {
        tail[buf[i]] -= 1;
        sa[tail[buf[i]]] = Some(i);
    }

    induce(buf, &mut sa, &count, &ls);

    let lms: Vec<_> = sa
        .into_iter()
        .map(std::option::Option::unwrap)
        .filter(|&i| ls[i] == SType(true))
        .collect(); // in lexicographic order
    let rs_sa = sa_is(&reduce(buf, &lms, &ls));

    // in appearing order
    let lms: Vec<_> = (0..len).filter(|&i| ls[i] == SType(true)).collect();

    let mut tail = bucket_tail(&count);
    let mut sa = vec![None; len];
    for i in rs_sa.into_iter().rev() {
        let j = lms[i];
        tail[buf[j]] -= 1;
        sa[tail[buf[j]]] = Some(j);
    }
    induce(buf, &mut sa, &count, &ls);

    sa.into_iter().map(std::option::Option::unwrap).collect()
}

impl<T: Ord> SuffixArray<T> {
    /// パターン検索を行う。
    ///
    /// # Complexity
    ///
    /// $O(|T|\\log(|S|))$ 時間。
    ///
    /// # Examples
    ///
    /// ```
    /// use nekolib::seq::SuffixArray;
    ///
    /// let s: Vec<_> = "abracadabra".chars().collect();
    /// let sa: SuffixArray<_> = s.into();
    ///
    /// assert_eq!(sa.search(&['a']).collect::<Vec<_>>(), vec![10, 7, 0, 3, 5]);
    /// assert_eq!(
    ///     sa.search(&"abra".chars().collect::<Vec<_>>()).nth(1),
    ///     Some(0)
    /// );
    /// assert_eq!(sa.search(&['a', 'e']).next(), None);
    /// ```
    pub fn search(&self, pat: &[T]) -> impl Iterator<Item = usize> + '_ {
        let lo = {
            let mut lt = 1_usize.wrapping_neg();
            let mut ge = self.sa.len();
            while ge.wrapping_sub(lt) > 1 {
                let mid = lt.wrapping_add(ge.wrapping_sub(lt) / 2);
                let pos = self.sa[mid];
                match self.buf[pos..].cmp(pat) {
                    Less => lt = mid,
                    _ => ge = mid,
                }
            }
            ge
        };
        if lo >= self.sa.len() {
            return self.sa[lo..lo].iter().cloned();
        }
        let hi = {
            let mut le = lo.wrapping_sub(1);
            let mut gt = self.sa.len();
            while gt.wrapping_sub(le) > 1 {
                let mid = le.wrapping_add(gt.wrapping_sub(le) / 2);
                let pos = self.sa[mid];
                let len = pat.len().min(self.buf[pos..].len());
                match self.buf[pos..pos + len].cmp(pat) {
                    Greater => gt = mid,
                    _ => le = mid,
                }
            }
            gt
        };
        self.sa[lo..hi].iter().cloned()
    }

    /// 高さ配列を返す。
    ///
    /// # Examples
    /// ```
    /// use nekolib::seq::SuffixArray;
    ///
    /// let s: Vec<_> = "abracadabra".chars().collect();
    /// let sa: SuffixArray<_> = s.into();
    /// assert_eq!(sa.lcpa(), [0, 0, 1, 4, 1, 1, 0, 3, 0, 0, 0, 2]);
    /// ```
    pub fn lcpa(&self) -> Vec<usize> {
        let n = self.buf.len();
        let mut rank = vec![0; n + 1];
        for i in 0..=n {
            rank[self.sa[i]] = i;
        }
        let mut h = 0;
        let mut res = vec![0; n + 1];
        for i in 0..n {
            let j = self.sa[rank[i] - 1];
            if h > 0 {
                h -= 1;
            }
            while j + h < n && i + h < n {
                if self.buf[j + h] != self.buf[i + h] {
                    break;
                }
                h += 1;
            }
            res[rank[i]] = h;
        }
        res
    }

    /// 自身を消費し、内部表現を返す。
    ///
    /// # Examples
    ///
    /// ```
    /// use nekolib::seq::SuffixArray;
    ///
    /// let s: Vec<_> = "abracadabra".chars().collect();
    /// let sa: SuffixArray<_> = s.into();
    /// let sa = sa.into_inner();
    /// assert_eq!(sa, vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
    /// ```
    pub fn into_inner(self) -> Vec<usize> {
        self.sa
    }
}

impl SuffixArray<char> {
    /// パターン文字列検索を行う。
    ///
    /// # Examples
    /// ```
    /// use nekolib::seq::SuffixArray;
    ///
    /// let sa: SuffixArray<_> = "abracadabra".to_string().into();
    /// let occ: Vec<_> = sa.search_str("ab").collect();
    /// assert_eq!(occ, vec![7, 0]);
    /// let occ: Vec<_> = sa.search_str("a").collect();
    /// assert_eq!(occ, vec![10, 7, 0, 3, 5]);
    /// assert_eq!(sa.search_str("e").next(), None);
    /// ```
    pub fn search_str(&self, pat: &str) -> impl Iterator<Item = usize> + '_ {
        let pat: Vec<_> = pat.chars().collect();
        self.search(&pat)
    }
}

impl<T: Ord> Index<usize> for SuffixArray<T> {
    type Output = usize;
    fn index(&self, i: usize) -> &usize {
        &self.sa[i]
    }
}

impl<T: Ord> From<SuffixArray<T>> for Vec<usize> {
    fn from(sa: SuffixArray<T>) -> Self {
        sa.sa
    }
}
