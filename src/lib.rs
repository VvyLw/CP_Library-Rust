pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod union_find;
use union_find::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

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
