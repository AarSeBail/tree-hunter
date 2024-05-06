use clap::{Parser, Subcommand};
use crate::mcts::{MctsArena, UCT};
use crate::tree_game::TreeGame;
use crate::bogo::BogoArena;
use crate::game::GameArena;
use crate::graph::laplacian::LapGraph;

mod graph;
mod mcts;
mod tree_game;
mod bogo;
mod game;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compute the spanning tree maximizer for some edge count
    #[clap(visible_alias("mcts"))]
    MonteCarloTreeSearch {
        edge_count: usize,
        upper_bound: usize,

        #[arg(short, long, default_value="0")]
        vertex_count: usize,

        #[arg(short, long, default_value="3.0")]
        exploration_parameter: f64,

        #[arg(short, long, default_value="10")]
        iterations: usize,

        #[arg(short, long, default_value="false")]
        verbose: bool,

        #[arg(short, long, default_value="0")]
        search_iterations: usize
    },

    /// Useful as a demonstration that MCTS is effective
    #[clap(visible_alias("bogo"))]
    BogoSearch {
        edge_count: usize,

        #[arg(short, long, default_value="0")]
        vertex_count: usize,

        #[arg(short, long, default_value="10")]
        iterations: usize,

        #[arg(short, long, default_value="0")]
        search_iterations: usize
    }
}

fn run_mcts(m: usize, n: usize, bound: usize, exploration: f64, search_iterations: usize, verbose: bool) -> usize {
    let b = 0;
    let g = TreeGame::<LapGraph>::new(m, n);
    let mut h = UCT::new(exploration);
    h.set_upper_score_bound(bound as f64);
    let mut a = MctsArena::new(g, h);
    for _j in 1..=search_iterations {
        for _i in 1..80 {
            a.play_round();
        }
        if a.prune() {
            println!("Algorithm terminated by searching all possible graphs. Best graph has {} spanning trees", b);
            break;
        }
    }
    if verbose {
        println!("Search Tree Size {}", a.tree_size());
        println!("Predicted Value {}", a.best() as usize);
        let q = a.best_game();
        print!("Graph Edges: ");
        q.print_graph();
    }
    a.best() as usize
}

fn run_bogo(m: usize, n: usize, search_iterations: usize) -> usize {
    let g = TreeGame::<LapGraph>::new(m, n);
    let mut a = BogoArena::new(g);
    for _j in 1..=search_iterations {
        for _i in 1..80 {
            a.play_round();
        }
        println!("{}", a.best());
    }
    a.best() as usize
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::MonteCarloTreeSearch {
                edge_count,
                upper_bound,
                mut vertex_count,
                exploration_parameter,
                iterations,
                verbose,
                mut search_iterations
             }) => {
            if vertex_count == 0 {
                vertex_count = *edge_count;
            }
            if search_iterations == 0 {
                search_iterations = 2usize.pow((*edge_count/2 - 1) as u32);
            }
            println!("Performing Monte-Carlo Tree Search");
            let mut v = Vec::with_capacity(*iterations);
            for i in 0..*iterations {
                if *verbose {
                    println!("---------------");
                }
                println!("Iteration {}", i + 1);
                v.push(run_mcts(
                    *edge_count,
                    vertex_count,
                    *upper_bound,
                    *exploration_parameter,
                    search_iterations,
                    *verbose,
                ));
            }
            let b = v.iter().max().unwrap();
            let count = v.iter().filter(|&s| *s == *b).count();
            println!("Value {} achieved in {count}/{} iterations ({}%)", b, *iterations,
                     100.0 * (count as f64/(*iterations as f64)));
        }

        Some(Commands::BogoSearch {
                 edge_count,
                 mut vertex_count,
                 iterations,
                 mut search_iterations
             }) => {
            if vertex_count == 0 {
                vertex_count = *edge_count;
            }
            if search_iterations == 0 {
                search_iterations = 2usize.pow((*edge_count/2 - 1) as u32);
            }
            println!("Performing BogoSearch");
            let mut v = Vec::with_capacity(*iterations);
            for i in 0..*iterations {
                println!("Iteration {i}");
                v.push(run_bogo(
                    *edge_count,
                    vertex_count,
                    search_iterations,
                ));
            }
            let b = v.iter().max().unwrap();
            let count = v.iter().filter(|&s| *s == *b).count();
            println!("Value {} achieved in {count}/{} iterations ({}%)", b, *iterations,
                     100.0 * (count as f64/(*iterations as f64)));
        }
        None => {}
    }

    /*let m = 17;
    let n = m - 1;
    let mut c = 0;
    let k = 30;
    let s = 7168;
    for _i in 1..=k {
        let b = run_mcts(m, n, s, 3.0);
        // let b = run_bogo(m, n);
        if b == s {
            c += 1;
        }
    }
    println!("{c}/{k}: {}%", 100.0*(c as f64/k as f64));*/
}
