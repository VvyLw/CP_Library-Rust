#![allow(unused_macros)]
macro_rules! out {
    ($x:expr) => {
        println!("{}", $x)
    };
}

macro_rules! rep {
    ($n: expr, $body: block) => {
        for _ in 0..$n {
            $body
        }
    };
    ($i: ident, $n: expr, $body: block) => {
        for $i in 0..$n {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $body: block) => {
        for $i in $a..=$b {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $c: expr, $body: block) => {
        let mut $i = $a;
        while $i <= $b {
            $body
            $i += $c;
        }
    };
}

macro_rules! rvp {
    ($i: ident, $n: expr, $body: block) => {
        for $i in (0..$n).rev() {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $body: block) => {
        for $i in ($b..=$a).rev() {
            $body
        }
    };
    ($i: ident, $a: expr, $b: expr, $c: expr, $body: block) => {
        let mut $i = $a;
        while $i >= $b {
            $body
            $i -= $c;
        }
    };
}

macro_rules! each {
    ($el: ident, $v: expr, $body: block) => {
        for $el in $v {
            $body
        }
    }
}

#[allow(dead_code)]
const MOD: i32 = 998244353;
#[allow(dead_code)]
const M0D: i32 = 1e9 as i32 + 7;
#[allow(dead_code)]
const INF: i32 = 1 << 30;
#[allow(dead_code)]
const LINF: i64 = (1 << 61) - 1;
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
const dx: [i32; 9] = [0, -1, 1, 0, 0, -1, -1, 1, 1];
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
const dy: [i32; 9] = [0, 0, 0, -1, 1, -1, 1, -1, 1];

#[allow(dead_code)]
mod union_find;
#[allow(unused_imports)]
use union_find::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uf_test1() {
        let mut uf = UnionFind::new(4);
        uf.unite(0, 1);
        assert!(uf.same(0, 1));
        uf.unite(1, 2);
        assert!(uf.same(0, 2));
        assert_eq!(uf.size(0), 3);
        assert!(!uf.same(0, 3));
        assert_eq!(uf.groups(), vec![vec![0, 1, 2], vec![3]]);
    }
    #[test]
    fn uf_test2() {
        let n = 9;
        let mut uf = UnionFind::new(n);
        uf.unite(1, 2);
        uf.unite(1, 3);
        uf.unite(1, 4);
        uf.unite(5, 6);
        uf.unite(6, 7);
        uf.unite(6, 8);
        assert_eq!(n - uf.groups().iter().map(|i| i.len()).max().unwrap(), 5);
    }
    #[test]
    fn uf_test3() {
        let n = 3;
        let mut uf = UnionFind::new(2 * n);
        uf.unite(0 + n, 1);
        uf.unite(0, 1 + n);
        uf.unite(1 + n, 2);
        uf.unite(1, 2 + n);
        assert!(is_bipartite(uf));
    }
}
