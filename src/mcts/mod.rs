use indextree::{Arena, NodeId};
use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;
use std::any::type_name;
use std::marker::PhantomData;
use crate::game::{Game, GameArena};

#[derive(Debug, Default)]
pub struct MctsNode {
    action: u64,
    best_rollout: f64,
    num_simulations: f64,
    total_accumulation: f64,

    // Heuristic to solve the multi-armed bandit problem
    is_expanded: bool,
    terminally_searched: bool,
}

pub struct MctsArena<G: Game, H: Heuristic> {
    arena: Arena<MctsNode>,
    root: NodeId,
    num_rollouts: usize,
    best_game: G,
    best_score: f64,
    game: G,
    rng: ThreadRng,
    heuristic: H,
    _p: PhantomData<G>,
}

impl<G: Game, H: Heuristic> MctsArena<G, H> {
    pub fn new(game: G, heuristic: H) -> Self {
        let mut arena = Arena::new();
        let root = arena.new_node(MctsNode::default());
        Self {
            arena,
            root,
            num_rollouts: 50,
            best_game: game.start(),
            best_score: f64::NEG_INFINITY,
            game,
            rng: rand::thread_rng(),
            heuristic,
            _p: PhantomData,
        }
    }

    fn select(&mut self) -> (NodeId, G) {
        let mut node_id = self.root;
        let mut game = self.game.start();
        while let Some(node) = self.arena.get(node_id) {
            if !node.get().is_expanded {
                break;
            }
            let mut best = f64::NEG_INFINITY;
            let mut next = node_id;
            // println!("Selecting");
            for id in node_id.children(&self.arena) {
                let h = self.heuristic.heuristic(self.arena[id].get(), self.arena[node_id].get());
                if h > best {
                    best = h;
                    next = id;
                }
                // println!("{h}");
            }

            // Reached a terminal node
            if best == f64::NEG_INFINITY {
                break;
            } else {
                node_id = next;
                game.act(self.arena.get(node_id).unwrap().get().action);
            }
        }
        let node = self.arena.get_mut(node_id).unwrap().get_mut();
        if !node.terminally_searched && game.is_terminal() {
            node.terminally_searched = true;
        }
        (node_id, game)
    }

    fn expand(&mut self, (parent, mut game): (NodeId, G)) -> (NodeId, G) {
        debug_assert!(
            !self.arena.get(parent).unwrap().get().is_expanded || game.is_terminal(),
            "MctsArena::{}::expand should not be run on expanded nodes",
            type_name::<G>()
        );
        self.arena.get_mut(parent).unwrap().get_mut().is_expanded = true;
        if game.is_terminal() {
            return (parent, game);
        }
        let actions = game.get_actions();
        let mut selected = parent;
        let chosen_index = self.rng.gen_range(0..actions.len());
        for (index, act) in actions.iter().enumerate() {
            let id = parent.append_value(
                MctsNode {
                    action: *act,
                    ..Default::default()
                },
                &mut self.arena,
            );
            if index == chosen_index {
                selected = id;
            }
        }
        game.act(actions[chosen_index]);
        return (selected, game);
    }

    fn rollout(&mut self, game: &mut G) -> f64 {
        while !game.is_terminal() {
            let action = *game.get_actions().choose(&mut rand::thread_rng()).unwrap();
            game.act(action);
        }
        game.get_score()
    }

    fn backpropagate(&mut self, selected: NodeId, best: f64, sum: f64) {
        let mut node_id = selected;
        loop {
            let node = self.arena.get_mut(node_id).unwrap().get_mut();
            node.best_rollout = f64::max(best, node.best_rollout);
            node.total_accumulation += sum;
            node.num_simulations += self.num_rollouts as f64;

            if let Some(next) = self.arena[node_id].parent() {
                node_id = next;
            } else {
                break;
            }
        }
        /*node_id = selected;
        while let Some(next) = self.arena[node_id].parent() {
            let nh = self.heuristic.heuristic(
                self.arena.get(node_id).unwrap().get(),
                self.arena.get(next).unwrap().get(),
            );
            self.arena.get_mut(node_id).unwrap().get_mut().heuristic = nh;
            node_id = next;
        }*/
    }

    fn terminate_leaves(&mut self, node_id: NodeId) -> bool {
        let node = self.arena.get(node_id).unwrap().get();
        if node.terminally_searched {
            true
        } else if node.is_expanded {
            let mut terminated = true;
            for id in node_id.children(&self.arena).collect::<Vec<_>>() {
                if self.terminate_leaves(id) {
                    id.remove(&mut self.arena);
                } else {
                    terminated = false;
                }
            }
            terminated
        } else {
            false
        }
    }

    pub fn prune(&mut self) -> bool {
        self.terminate_leaves(self.root)
    }

    pub fn tree_size(&self) -> usize {
        self.arena.count()
    }

    #[allow(dead_code)]
    pub(crate) fn print(&self) {
        let p = self.root.debug_pretty_print(&self.arena);
        println!("{:?}", p);
    }

    #[allow(dead_code)]
    pub(crate) fn select_route(&mut self) -> Vec<u64> {
        let mut v = vec![];
        let (n, g) = self.select();
        println!("{}", g.is_terminal());
        println!("{}", g.get_score());
        println!("{}", self.arena[n].get().best_rollout);
        for a in n.ancestors(&self.arena) {
            v.push(self.arena[a].get().action);
        }
        v.reverse();
        v
    }

    pub(crate) fn best_game(&self) -> G {
        self.best_game.clone()
    }
}

impl<G: Game, H: Heuristic> GameArena<G> for MctsArena<G, H> {
    fn play_round(&mut self) {
        let p = self.select();
        let (parent, game) = self.expand(p);
        let mut best = 0.0;
        let mut sum = 0.0;
        for _i in 1..=self.num_rollouts {
            let mut g = game.clone();
            let val = self.rollout(&mut g);
            if val > best {
                best = val;
            }
            if val > self.best_score {
                self.best_game = g.clone();
                self.best_score = val;
            }
            sum += val;
        }
        self.heuristic.update_heuristic(sum, best, self.num_rollouts);
        self.backpropagate(parent, best, sum);
    }

    fn best(&self) -> f64 {
        self.best_score
    }
}

pub trait Heuristic {
    fn heuristic(&self, node: &MctsNode, parent: &MctsNode) -> f64;
    fn update_heuristic(&mut self, rollout_sum: f64, best: f64, new_rollouts: usize);
}

pub struct UCT {
    pub(crate) exploration: f64,
    num_rollouts: f64,
    mean_rollout: f64,
    best_rollout: f64,
    upper_bound: f64,
}

impl UCT {
    pub(crate) fn new(exploration: f64) -> Self {
        Self {
            exploration,
            num_rollouts: 0.0,
            mean_rollout: 0.0,
            best_rollout: 1.0,
            upper_bound: 1.0,
        }
    }

    pub(crate) fn set_upper_score_bound(&mut self, bound: f64) {
        self.upper_bound = bound;
    }
}

impl Heuristic for UCT {
    fn heuristic(&self, node: &MctsNode, parent: &MctsNode) -> f64 {
        if node.num_simulations == 0.0 {
            // Also tried return 1.0
            // Returning infinity is more sensible and seems to give better results
            return f64::INFINITY;
            // return 1.0;
        }

        /*node.total_accumulation / (self.upper_bound * node.num_simulations)
            + self.exploration * (parent.num_simulations.ln() / node.num_simulations).sqrt()*/
        node.best_rollout / self.upper_bound
            + self.exploration * (parent.num_simulations.ln() / node.num_simulations).sqrt()
        /*node.best_rollout / self.best_rollout
            + self.exploration * (parent.num_simulations.ln() / node.num_simulations).sqrt()*/
    }

    fn update_heuristic(&mut self, rollout_sum: f64, best: f64, new_rollouts: usize) {
        self.mean_rollout *= self.num_rollouts / (self.num_rollouts + new_rollouts as f64);
        self.num_rollouts += new_rollouts as f64;
        self.mean_rollout += rollout_sum / self.num_rollouts;
        self.best_rollout = self.best_rollout.max(best);
    }
}