use super::*;

pub type ProcRate = u32;

#[derive(Debug, Clone, PartialEq)]
pub enum SkillTrigger {
    PROC(ProcRate),
    BasedOnCard(SkillTriggerValueFormula),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillTriggerValueFormula{
    TriggerBasedOnCardLevel(Percentage),
    TriggerBasedOnCardTier(Percentage)
}