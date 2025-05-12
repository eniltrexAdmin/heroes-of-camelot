use crate::macroquad::*;
use crate::macroquad::textures_loader::local_fs_card_textures_repository::LocalCardTexturesRepository;

pub trait State {
    fn debug(&self) -> &str;
    fn update(&mut self) -> StateTransition;
    fn draw(&self);
}

pub enum StateTransition {
    None,
    Pop,
    Push(HocStates),
    Switch(HocStates)
}

pub enum HocStates{
    Presentation,
    Battle,
    EndBattle
}

pub struct StackState{
    states: Vec<Box<dyn State>>,
    textures_repository: LocalCardTexturesRepository
}


impl StackState {
    pub fn new(textures_repository: LocalCardTexturesRepository) -> Self {
        Self { states: Vec::new(), textures_repository }
    }

    pub fn push(&mut self, state: Box<dyn State>) {
        self.states.push(state);
    }

    pub fn pop(&mut self) {
        self.states.pop();
    }

    pub async fn update(&mut self) {
        if let Some(top_state) = self.states.last_mut() {
            match top_state.update() {
                StateTransition::None => {},
                StateTransition::Pop => {self.pop();},
                StateTransition::Push(state) => {
                    self.states.push(self.create_state(state).await);
                },
                StateTransition::Switch(state) => {
                    self.pop();
                    self.states.push(self.create_state(state).await);
                }
            }
        }
    }

    async fn create_state(&self, state: HocStates) -> Box<dyn State> {
        match state{
            HocStates::Presentation => {
                Box::new(PresentationState::new())
            },
            HocStates::Battle => {
                Box::new(BattleState::new(&self.textures_repository).await)
            },
            HocStates::EndBattle => {
                Box::new(EndBattleState{})
            }
        }
    }

    pub fn draw(&self) {
        for state in &self.states {
            state.draw();
        }
    }
}