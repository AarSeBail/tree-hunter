/*
 * The goal of this module is to provide evidence that MCTS is better than a random search
 */

use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use crate::game::{Game, GameArena};

pub struct BogoArena<G: Game> {
    game: G,
    best_score: f64,
    rng: ThreadRng
}

impl<G: Game> BogoArena<G> {
    pub fn new(game: G) -> Self {
        Self {
            game,
            best_score: f64::NEG_INFINITY,
            rng: rand::thread_rng()
        }
    }
}

impl<G: Game> GameArena<G> for BogoArena<G> {
    fn play_round(&mut self) {
        let mut g = self.game.clone();
        while !g.is_terminal() {
            let act = *g.get_actions().choose(&mut self.rng).unwrap();
            g.act(act);
        }
        if g.get_score() > self.best_score {
            self.best_score = g.get_score();
        }
    }

    fn best(&self) -> f64 {
        self.best_score
    }
}