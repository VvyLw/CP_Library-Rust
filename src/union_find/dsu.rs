pub trait DSU {
    fn new(n: usize) -> Self;
    fn root(&mut self, i: usize) -> usize;
    fn size(&mut self, i: usize) -> usize;
    fn unite(&mut self, i: usize, j: usize) -> bool;
    fn same(&mut self, i: usize, j: usize) -> bool;
    fn groups(&mut self) -> Vec<Vec<usize>>;
}