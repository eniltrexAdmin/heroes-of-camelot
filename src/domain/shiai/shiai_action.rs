use crate::domain::shiai::attack::AttackCommand;
use crate::domain::ShiaiEvent;


#[derive(Clone, Debug)]
pub enum ShiaiCommand {
    Attack(AttackCommand)
}

#[derive(Clone, Debug)]
pub struct ShiaiAction{
    pub command: ShiaiCommand,
    pub events: Vec<ShiaiEvent>,
}

impl ShiaiAction{
    pub fn new(command: ShiaiCommand, events: Vec<ShiaiEvent>) -> Self {
        Self{command, events}
    }
}

