pub mod laplacian;

// Implementation note: isolated vertices are not real and cannot hurt you
pub trait Graph: Clone {
    fn empty(vertex_count: usize) -> Self;
    fn complete(vertex_count: usize) -> Self;
    fn add_edge(&mut self, i: usize, j: usize);
    fn order(&self) -> usize;
    fn size(&self) -> usize;
    fn spanning_tree_count(&self) -> usize;
    fn degree(&self, vertex: usize) -> usize;
    fn lowest_free_vertex(&self) -> Option<usize>;
    fn print_edges(&self);
}
