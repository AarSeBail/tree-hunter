use nalgebra::{DMatrix, Dyn, OMatrix};
use crate::graph::Graph;

#[derive(Debug, Clone)]
struct LapGraph {
    laplacian: OMatrix<f64, Dyn, Dyn>,
    vertex_count: usize
}

impl Graph for LapGraph {
    fn empty(vertex_count: usize) -> Self {
        Self {
            laplacian: DMatrix::<f64>::zeros(vertex_count, vertex_count),
            vertex_count
        }
    }

    fn complete(vertex_count: usize) -> Self {
        let mut laplacian = DMatrix::<f64>::from_element(vertex_count, vertex_count, -1.0);

        laplacian.fill_diagonal((vertex_count as f64) - 1.0);

        Self {
            laplacian,
            vertex_count
        }
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        debug_assert!(
            i != j,
            "LapGraph::add_edge does not support self loops"
        );

        debug_assert!(
            i < self.vertex_count && j < self.vertex_count,
            "LapGraph::add_edge indices must lie in [0, {})",
            self.vertex_count
        );

        debug_assert!(
            self.laplacian[(i, j)] == 0.0,
            "LapGraph::add_edge does not support multi edges"
        );

        self.laplacian[(i, j)] -= 1.0;
        self.laplacian[(j, i)] -= 1.0;
        self.laplacian[(i, i)] += 1.0;
        self.laplacian[(j, j)] += 1.0;
    }

    fn order(&self) -> usize {
        self.laplacian.diagonal().iter().filter(|&&x| x != 0.0).count()
        // self.vertex_count
    }

    fn size(&self) -> usize {
        (self.laplacian.trace() / 2.0) as usize
    }

    fn spanning_tree_count(&self) -> usize {
        let eigen = self.laplacian.symmetric_eigenvalues();
        let mut p = 1.0;
        for i in 1..eigen.len() {
            p *= eigen[i];
        }
        p as usize / eigen.len()
    }
}