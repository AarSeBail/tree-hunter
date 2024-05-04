use indextree::{Arena, NodeId};

mod tree_game;

trait Game: Clone + Default {
    fn get_actions(&self) -> Vec<u64>;
    fn is_terminal(&self) -> bool;
    fn act(&mut self, action: u64) -> bool;
    fn get_score(&self) -> f64;
}

#[derive(Default)]
struct MctsNode {
    my_move: u64,
    best_rollout: f64,
    num_simulations: f64,
    total_accumulation: f64,
    // Heuristic to solve the multi-armed bandit problem
    heuristic: f64,
    is_expanded: bool
}

struct MctsArena<G: Game> {
    arena: Arena<MctsNode>,
    root: NodeId
}

impl<G: Game> MctsArena<G> {
    fn new() -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node(MctsNode::default());
        Self {
            arena,
            root
        }
    }

    fn select(&self) -> NodeId {
        let mut node_id = self.root;
        while let Some(node) = self.arena.get(node_id) {
            if !node.get().is_expanded {
                break
            }
            let best = f64::NEG_INFINITY;
            for id in node_id.children(&self.arena) {
                if self.arena.get(id).unwrap().get().heuristic > best {
                    node_id = id;
                }
            }
        }
        node_id
    }

    fn mcts_round(&mut self) {

    }
}