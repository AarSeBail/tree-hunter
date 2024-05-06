pub trait Game: Clone {
    fn get_actions(&self) -> Vec<u64>;
    fn is_terminal(&self) -> bool;
    fn act(&mut self, action: u64) -> bool;
    fn get_score(&self) -> f64;
    fn start(&self) -> Self;
}

pub trait GameArena<G: Game> {
    fn play_round(&mut self);
    fn best(&self) -> f64;
}