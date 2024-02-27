pub(crate) mod dsu;
use crate::union_find::dsu::DSU;

pub struct UnionFind {
    par: Vec<i32>
}
impl DSU for UnionFind {
    fn new(n: usize) -> Self {
        Self { par: vec![-1; n] }
    }
    fn root(&mut self, i: usize) -> usize {
        if self.par[i] >= 0 {
            self.par[i] = self.root(self.par[i] as usize) as i32;
            self.par[i] as usize
        } else {
            i
        }
    }
    fn size(&mut self, i: usize) -> usize {
        let id = self.root(i);
        -self.par[id] as usize as usize
    }
    fn unite(&mut self, mut i: usize, mut j: usize) -> bool {
        i = self.root(i);
        j = self.root(j);
        if i == j {
            return false;
        }
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }
        self.par[i] += self.par[j];
        self.par[j] = i as i32;
        true
    }
    fn same(&mut self, i: usize, j: usize) -> bool {
        self.root(i) == self.root(j)
    }
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.par.len();
        let mut res = vec![Vec::new(); n];
        for i in 0..n {
            res[self.root(i)].push(i);
        }
        res.into_iter().filter(|g| !g.is_empty()).collect()
    }
}

pub fn is_bipartite(mut uf: UnionFind) -> bool {
    assert_eq!(uf.par.len() % 2, 0);
    let n = uf.par.len() / 2;
    let mut ok = true;
    for i in 0..n {
        ok &= uf.root(i) != uf.root(i + n);
    }
    ok
}

pub struct WeightedUnionFind {
    par: Vec<i32>,
    weight: Vec<i64>
}
impl DSU for WeightedUnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: vec![-1; n],
            weight: vec![0; n]
        }
    }
    fn root(&mut self, i: usize) -> usize {
        if self.par[i] < 0 {
            return i
        }
        let r = self.root(self.par[i] as usize);
        self.weight[i] += self.weight[self.par[i] as usize];
        self.par[i] = r as i32;
        self.par[i] as usize
    }
    fn size(&mut self, i: usize) -> usize {
        let id = self.root(i);
        -self.par[id] as usize as usize
    }
    #[deny(useless_deprecated)]
    fn unite(&mut self, i: usize, j: usize) -> bool {
        self.unite(i, j, 0) > 0
    }
    fn same(&mut self, i: usize, j: usize) -> bool {
        self.root(i) == self.root(j)
    }
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.par.len();
        let mut res = vec![Vec::new(); n];
        for i in 0..n {
            res[self.root(i)].push(i);
        }
        res.into_iter().filter(|g| !g.is_empty()).collect()
    }
}
impl WeightedUnionFind {
    pub fn get(&mut self, i: usize) -> i64 {
        self.root(i);
        self.weight[i]
    }
    pub fn diff(&mut self, i: usize, j: usize) -> i64 {
        self.get(j) - self.get(i)
    }
    pub fn unite(&mut self, mut i: usize, mut j: usize, mut w: i64) -> i32 {
        w += self.diff(j, i);
        i = self.root(i);
        j = self.root(j);
        if i == j {
            return if w == 0 {
                0
            } else {
                -1
            }
        }
        if i > j {
            std::mem::swap(&mut i, &mut j);
            w = -w;
        }
        self.par[i] += self.par[j];
        self.par[j] = i as i32;
        self.weight[j] = w;
        1
    }
}