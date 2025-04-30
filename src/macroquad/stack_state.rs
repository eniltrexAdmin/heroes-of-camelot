pub trait State {
    fn update(&mut self) -> StateTransition;
    fn draw(&self);
    fn debug(&self) -> &str;
}

pub enum StateTransition {
    None,
    Pop,
    Push(Box<dyn State>)
}

pub struct StackState{
    states: Vec<Box<dyn State>>,
}


impl StackState {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    pub fn push(&mut self, state: Box<dyn State>) {
        self.states.push(state);
    }

    pub fn pop(&mut self) {
        self.states.pop();
    }

    pub fn update(&mut self) {
        if let Some(top_state) = self.states.last_mut() {
            match top_state.update() {
                StateTransition::None => {},
                StateTransition::Pop => {self.pop();},
                StateTransition::Push(state) => {
                    self.states.push(state);
                }
            }
        }
    }

    pub fn draw(&self) {
        for state in &self.states {
            state.draw();
        }
    }
}