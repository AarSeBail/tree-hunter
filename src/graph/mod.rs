pub mod laplacian;

pub struct Vertex(i64);
pub struct Edge(Vertex, Vertex);

// Implementation note: isolated vertices are not real and cannot hurt you
pub trait Graph {
    fn empty(vertex_count: usize) -> Self;
    fn complete(vertex_count: usize) -> Self;
    fn add_edge(&mut self, i: usize, j: usize);
    fn order(&self) -> usize;
    fn size(&self) -> usize;
    fn spanning_tree_count(&self) -> usize;
}