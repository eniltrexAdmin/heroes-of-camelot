use crate::domain::*;

pub fn perform_skill(state: ShiaiState, attacker_pos: ShiaiPosition)
                     -> (ShiaiState, Vec<ShiaiEvent>) {

    let new_state = state.apply_domain_events(
        vec![]);

    (new_state, vec![])

}