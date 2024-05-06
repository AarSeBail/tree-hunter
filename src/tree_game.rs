use crate::game::Game;
use crate::graph::Graph;

#[derive(Clone, Debug)]
pub struct TreeGame<G: Graph> {
    max_edges: usize,
    num_vertices: usize,
    num_actions: usize,
    max_actions: usize,
    num_edges_added: usize,
    graph: G,
    current_edge: (usize, usize),
}

impl<G: Graph> TreeGame<G> {
    pub fn new(m: usize, n: usize) -> Self {
        Self {
            max_edges: m,
            num_vertices: n,
            num_actions: 0,
            max_actions: n * (n - 1) / 2,
            num_edges_added: 0,
            graph: G::empty(n),
            // .0 > .1
            current_edge: (1, 0),
        }
    }

    pub fn print_graph(&self) {
        self.graph.print_edges();
    }
}

impl<G: Graph> Game for TreeGame<G> {
    fn get_actions(&self) -> Vec<u64> {
        if self.num_edges_added >= self.max_edges || self.num_actions >= self.max_actions {
            return vec![];
        }
        if self.num_edges_added == 0 {
            return vec![1];
        }

        let d1 = self.graph.degree(self.current_edge.0);
        let d2 = self.graph.degree(self.current_edge.1);
        if d1 == 0 && d2 == 0 {
            return vec![0, 1];
        }
        if d1 == 0 {
            if self.current_edge.0 == self.graph.lowest_free_vertex().unwrap() {
                vec![0, 1]
            }else {
                vec![0]
            }
        }else if d2 == 0 {
            if self.current_edge.1 == self.graph.lowest_free_vertex().unwrap() {
                vec![0, 1]
            }else {
                vec![0]
            }
        }else {
            vec![0, 1]
        }
        // vec![0, 1]
    }

    fn is_terminal(&self) -> bool {
        self.num_edges_added >= self.max_edges || self.num_actions >= self.max_actions
    }

    fn act(&mut self, action: u64) -> bool {
        if action != 0 && action != 1 {
            return false;
        }
        if self.num_edges_added >= self.max_edges {
            return false;
        }
        if self.num_actions >= self.max_actions {
            return false;
        }
        if action == 1 {
            self.num_edges_added += 1;
            self.graph
                .add_edge(self.current_edge.0, self.current_edge.1);
        }

        self.current_edge.0 += 1;
        if self.current_edge.0 >= self.num_vertices {
            self.current_edge.0 = self.current_edge.1 + 2;
            self.current_edge.1 += 1;
        }

        self.num_actions += 1;

        return true;
    }

    fn get_score(&self) -> f64 {
        self.graph.spanning_tree_count() as f64
    }

    fn start(&self) -> Self {
        Self {
            max_edges: self.max_edges,
            num_vertices: self.num_vertices,
            num_edges_added: 0,
            num_actions: 0,
            max_actions: self.max_actions,
            graph: G::empty(self.num_vertices),
            current_edge: (1, 0),
        }
    }
}
