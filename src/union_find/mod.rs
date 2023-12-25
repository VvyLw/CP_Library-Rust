pub struct UnionFind {
    par: Vec<i32>
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { par: vec![-1; n] }
    }
    pub fn root(&mut self, i: usize) -> usize {
        if self.par[i] >= 0 {
            self.par[i] = self.root(self.par[i] as usize) as i32;
            self.par[i] as usize
        } else {
            i
        }
    }
    pub fn size(&mut self, i: usize) -> i32 {
        let id = self.root(i);
        -self.par[id]
    }
    pub fn unite(&mut self, mut i: usize, mut j: usize) -> bool {
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
    pub fn same(&mut self, i: usize, j: usize) -> bool {
        self.root(i) == self.root(j)
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.par.len();
        let mut res = vec![Vec::new(); n];
        for i in 0..n {
            res[self.root(i)].push(i);
        }
        res.into_iter()
            .filter(|g| !g.is_empty())
            .collect::<Vec<Vec<usize>>>()
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