use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        _:usize,
        q:usize,
        s:Chars,
        clr:[(u8, Usize1, Usize1);q],
    }
    let first = s.into_iter().map(|c| Node::new(c)).collect::<Vec<_>>();
    let mut seg_tree = LazySegTree::new(first, merge, update, composition);
    for (c, l, r) in clr {
        if c == 1 {
            seg_tree.range_update(l, r, Param::new());
        } else {
            println!("{}", seg_tree.get_range(l, r).max1);
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Node {
    l0: usize,
    l1: usize,
    r0: usize,
    r1: usize,
    max0: usize,
    max1: usize,
    len: usize,
}

impl Node {
    fn new(c: char) -> Self {
        if c == '0' {
            return Self {
                l0: 1,
                l1: 0,
                r0: 1,
                r1: 0,
                max0: 1,
                max1: 0,
                len: 1,
            };
        } else {
            return Self {
                l0: 0,
                l1: 1,
                r0: 0,
                r1: 1,
                max0: 0,
                max1: 1,
                len: 1,
            };
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Param {
    p: bool,
}

impl Param {
    pub fn new() -> Self {
        Self { p: true }
    }
}

fn merge(p1: Node, p2: Node) -> Node {
    let l0 = if p1.l0 == p1.len {
        p1.l0 + p2.l0
    } else {
        p1.l0
    };
    let l1 = if p1.l1 == p1.len {
        p1.l1 + p2.l1
    } else {
        p1.l1
    };
    let r0 = if p2.r0 == p2.len {
        p1.r0 + p2.r0
    } else {
        p2.r0
    };
    let r1 = if p2.r1 == p2.len {
        p1.r1 + p2.r1
    } else {
        p2.r1
    };
    let max0 = p1.max0.max(p2.max0).max(l0).max(r0).max(p1.r0 + p2.l0);
    let max1 = p1.max1.max(p2.max1).max(l1).max(r1).max(p1.r1 + p2.l1);
    let len = p1.len + p2.len;
    Node {
        l0,
        l1,
        r0,
        r1,
        max0,
        max1,
        len,
    }
}

// p: 更新前
// len: ノードiがカバーする区間の長さ
// params 更新に使うパラメータ
// 更新結果のノードiの値を返却する
fn update(p: Node, _len: usize, params: Param) -> Node {
    if params.p {
        return Node {
            l0: p.l1,
            l1: p.l0,
            r0: p.r1,
            r1: p.r0,
            max0: p.max1,
            max1: p.max0,
            len: p.len,
        };
    }
    p
}

// 先に適用する更新のパラメータ
// 次に適用する更新のパラメータ
fn composition(p1: Param, p2: Param) -> Param {
    Param { p: p1.p ^ p2.p }
}

pub struct LazySegTree<T, F, L, P, C>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
    L: Fn(T, usize, P) -> T,
    P: Clone + Copy,
    C: Fn(P, P) -> P,
{
    values: Vec<Option<T>>,
    ranges: Vec<Option<(usize, usize)>>,
    lazy_params: Vec<Option<P>>,
    operator: F,
    lazy_operator: L,
    composition: C,
}

impl<T, F, L, P, C> LazySegTree<T, F, L, P, C>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
    L: Fn(T, usize, P) -> T,
    P: Clone + Copy,
    C: Fn(P, P) -> P,
{
    pub fn new(values: Vec<T>, operator: F, lazy_operator: L, composition: C) -> Self {
        let size = values.len();
        let tree_size = 2 * size.next_power_of_two() - 1;
        let vals = vec![None; tree_size];
        let ranges = vec![None; tree_size];

        let mut seg_tree = Self {
            values: vals,
            ranges,
            lazy_params: vec![None; tree_size],
            operator,
            lazy_operator,
            composition,
        };

        for i in 0..size {
            let index_of_tree = seg_tree.index_of_tree(i);
            seg_tree.values[index_of_tree] = Some(values[i]);
            seg_tree.ranges[index_of_tree] = Some((i, i));
        }

        for i in 0..seg_tree.index_of_tree(0) {
            // 降順に更新
            let index = seg_tree.index_of_tree(0) - 1 - i;
            let children_index = seg_tree.children_index(index).unwrap();
            let v1 = seg_tree.values[children_index.0];
            let v2 = seg_tree.values[children_index.1];
            let val = seg_tree.eval(v1, v2);

            let range = if seg_tree.ranges[children_index.0].is_none() {
                None
            } else {
                let range_min = seg_tree.ranges[children_index.0].unwrap().0;
                let range_max = if seg_tree.ranges[children_index.1].is_none() {
                    seg_tree.ranges[children_index.0].unwrap().1
                } else {
                    seg_tree.ranges[children_index.1].unwrap().1
                };
                Some((range_min, range_max))
            };

            seg_tree.values[index] = val;
            seg_tree.ranges[index] = range;
        }

        seg_tree
    }

    pub fn get(&mut self, index: usize) -> T {
        self.get_range(index, index)
    }

    pub fn get_range(&mut self, left: usize, right: usize) -> T {
        self.get_range_sub(left, right, 0).unwrap()
    }

    fn get_range_sub(&mut self, left: usize, right: usize, index: usize) -> Option<T> {
        if self.ranges[index].is_none() {
            // 指定されたindexに要素が存在しない
            None
        } else {
            let current_range = self.ranges[index].unwrap();

            // 遅延評価
            self.lazy_eval(index);

            if self.children_index(index).is_none() {
                // 葉
                if left <= current_range.0 && current_range.0 <= right {
                    self.values[index]
                } else {
                    None
                }
            } else if left <= current_range.0 && current_range.1 <= right {
                // 現在の範囲が覆われている場合
                // 現在の範囲での値を返す
                self.values[index]
            } else if right < current_range.0 || current_range.1 < left {
                // 現在の範囲と共通部分がない
                None
            } else {
                // 現在の範囲と共通部分がある場合
                // 子供も調べる
                let val_left =
                    self.get_range_sub(left, right, self.children_index(index).unwrap().0);
                let val_right =
                    self.get_range_sub(left, right, self.children_index(index).unwrap().1);
                self.eval(val_left, val_right)
            }
        }
    }

    pub fn update(&mut self, index: usize, value: T) {
        // 遅延評価を実行する
        self.get(index);

        let mut index = self.index_of_tree(index);
        self.values[index] = Some(value);

        while !self.parent_index(index).is_none() {
            index = self.parent_index(index).unwrap();
            let children = self.children_index(index).unwrap();
            self.values[index] = self.eval(self.values[children.0], self.values[children.1]);
        }
    }

    pub fn range_update(&mut self, left: usize, right: usize, params: P) {
        // 該当のノードの遅延情報を解決させるため、先にこの範囲で取得する
        self.get_range(left, right);

        self.range_update_sub(left, right, params, 0);
    }

    fn range_update_sub(&mut self, left: usize, right: usize, params: P, index: usize) {
        if let Some(current_range) = self.ranges[index] {
            if right < current_range.0 || current_range.1 < left {
                // 範囲外なら何もしない
                return;
            } else if left <= current_range.0 && current_range.1 <= right {
                // 今回の遅延情報を入れる
                self.lazy_params[index] = Some(params);

                // 親ノードの更新の簡単のため遅延評価を解決する
                self.lazy_eval(index);
            } else {
                // 子ノードから計算済みの値をもらう
                if let Some((index_c1, index_c2)) = self.children_index(index) {
                    let range_c1 = self.ranges[index_c1];
                    let range_c2 = self.ranges[index_c2];

                    if let Some(range_c1) = range_c1 {
                        let intersection_left = if left <= range_c1.0 { range_c1.0 } else { left };
                        let intersection_right = if right <= range_c1.1 {
                            right
                        } else {
                            range_c1.1
                        };

                        if intersection_left <= intersection_right {
                            // 共通部分がある
                            self.range_update_sub(
                                intersection_left,
                                intersection_right,
                                params,
                                index_c1,
                            );
                        }
                    }

                    if let Some(range_c2) = range_c2 {
                        let intersection_left = if left <= range_c2.0 { range_c2.0 } else { left };
                        let intersection_right = if right <= range_c2.1 {
                            right
                        } else {
                            range_c2.1
                        };
                        if intersection_left <= intersection_right {
                            // 共通部分がある
                            self.range_update_sub(
                                intersection_left,
                                intersection_right,
                                params,
                                index_c2,
                            );
                        }
                    }
                    if self.values[index_c1].is_some() {
                        self.lazy_eval(index_c1);
                    }

                    if self.values[index_c2].is_some() {
                        self.lazy_eval(index_c2);
                    }
                    self.values[index] = self.eval(self.values[index_c1], self.values[index_c2]);
                }
            }
        }
    }

    fn lazy_eval(&mut self, index: usize) -> Option<T> {
        let current_range = self.ranges[index].unwrap();
        if let Some(params) = self.lazy_params[index] {
            let evaluated_value = (self.lazy_operator)(
                self.values[index].unwrap(),
                current_range.1 - current_range.0 + 1,
                params,
            );
            self.values[index] = Some(evaluated_value);
            self.lazy_params[index] = None;

            if let Some((index_c1, index_c2)) = self.children_index(index) {
                // 子に伝播する
                let range_c1 = self.ranges[index_c1];
                let range_c2 = self.ranges[index_c2];

                if let Some(_) = range_c1 {
                    if let Some(first_params) = self.lazy_params[index_c1] {
                        self.lazy_params[index_c1] = Some((self.composition)(first_params, params));
                    } else {
                        self.lazy_params[index_c1] = Some(params);
                    }
                }

                if let Some(_) = range_c2 {
                    if let Some(first_params) = self.lazy_params[index_c2] {
                        self.lazy_params[index_c2] = Some((self.composition)(first_params, params));
                    } else {
                        self.lazy_params[index_c2] = Some(params);
                    }
                }
            }
        }

        self.values[index]
    }

    fn eval(&self, v1: Option<T>, v2: Option<T>) -> Option<T> {
        if v1.is_none() {
            if v2.is_none() {
                None
            } else {
                v2
            }
        } else {
            if v2.is_none() {
                v1
            } else {
                Some((self.operator)(v1.unwrap(), v2.unwrap()))
            }
        }
    }

    fn children_index(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.values.len() / 2 {
            None
        } else {
            Some((2 * index + 1, 2 * index + 2))
        }
    }

    fn parent_index(&self, index: usize) -> Option<usize> {
        if index == 0 {
            None
        } else {
            Some((index - 1) / 2)
        }
    }

    fn index_of_tree(&self, index: usize) -> usize {
        self.values.len() / 2 + index
    }
}
