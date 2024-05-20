
#[derive(Debug, Clone, Copy)]
pub struct GameInputState {
    pub p1_paddle_state: PaddleState,
    pub p2_paddle_state: PaddleState,
}

impl Default for GameInputState {
    fn default() -> Self {
        GameInputState { 
            p1_paddle_state: PaddleState::Stopped,
            p2_paddle_state: PaddleState::Stopped,  }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum PaddleState {
    Stopped,
    MovingUp,
    MovingDown,
}