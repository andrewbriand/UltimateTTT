use std::time::Duration;
use async_trait::async_trait;

#[async_trait]
pub trait AI {
    // tells the AI to get ready and return whether it was able to 
    // get ready in secs_allowed seconds
    async fn ready(&mut self, time_allowed: Duration) -> bool;
    
    // returns the move the AI wants to make
    // given that the last move was last_move
    // last_move should be -1 if this is the first move of the game
    // get_move returns -1 if the ai wants to resign
    async fn get_move(&mut self, last_move : i64, rem_time_x: Duration, rem_time_o: Duration) -> i64;

    fn get_rem_time(&self) -> Duration;

    fn cleanup(&mut self);
}