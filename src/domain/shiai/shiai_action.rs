use crate::domain::shiai::attack::AttackCommand;
use crate::domain::ShiaiEvent;


#[derive(Clone)]
pub enum ShiaiCommand {
    Attack(AttackCommand)
}

#[derive(Clone)]
pub struct ShiaiAction{
    pub command: ShiaiCommand,
    pub events: Vec<ShiaiEvent>,
}

impl ShiaiAction{
    pub fn new(command: ShiaiCommand, events: Vec<ShiaiEvent>) -> Self {
        Self{command, events}
    }
}

