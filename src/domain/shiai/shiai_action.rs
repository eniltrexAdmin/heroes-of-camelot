use crate::domain::*;

#[derive(Clone, Debug)]
pub struct ShiaiAction{
    pub subject: ShiaiPosition,
    pub command: ShiaiCommandType,
    pub events: Vec<ShiaiEvent>,
}


#[derive(Clone, Debug)]
pub enum ShiaiCommandType {
    Attack
}

#[derive(Clone, Debug)]
pub struct ShiaiEvent {
    pub target: ShiaiPosition,
    pub event: ShiaiEventType
}

#[derive(Clone, Debug)]
pub enum ShiaiEventType {
    // AttackReceived(BattleTeamAttack),
    DamageReceived(Damage),
}

impl ShiaiEvent {
    pub fn new_damage_received(target: ShiaiPosition, damage: Damage) -> Self {
        Self{
            target,
            event: ShiaiEventType::DamageReceived(damage)
        }
    }
}
