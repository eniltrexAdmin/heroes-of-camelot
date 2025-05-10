use crate::domain::{ShiaiAction, ShiaiError};
use crate::domain::shiai::shiai_state::ShiaiState;

#[derive(Clone)]
pub struct ShiaiTurn{
    pub actions: Vec<ShiaiAction>,
    pub state_result: ShiaiState
}

impl ShiaiTurn{
    pub fn new(actions: Vec<ShiaiAction>, mut shiai_state: ShiaiState) -> Result<Self, ShiaiError> {
        for action in actions.clone() {
            shiai_state = shiai_state.apply_domain_events(action.events)?
        }

        Ok(Self{
            actions,
            state_result: shiai_state
        })
    }
}