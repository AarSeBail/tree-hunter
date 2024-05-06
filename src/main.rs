use crate::graph::laplacian::LapGraph;
use crate::graph::Graph;
use crate::mcts::{Game, MctsArena, UCT};
use crate::tree_game::TreeGame;
use indextree::Arena;

mod graph;
mod mcts;
mod tree_game;

fn main() {
    let m = 17;
    let n = m - 1;
    let b = 0;
    let g = TreeGame::<LapGraph>::new(m, n);
    let mut h = UCT::new(1.414);
    h.set_upper_score_bound(7168.0);
    let mut a = MctsArena::new(g, h);
    for _j in 1..4 * m {
        for _i in 1..1000 {
            a.mcts_round();
        }
        println!("{}", a.best());
        if a.prune() {
            println!("Algorithm terminated by searching all possible graphs. Best graph has {} spanning trees", b);
            break;
        }
    }
    println!("Search Tree Size {}", a.tree_size());
    // a.print();
    let mut q = a.best_graph();
    q.print_graph();
}
