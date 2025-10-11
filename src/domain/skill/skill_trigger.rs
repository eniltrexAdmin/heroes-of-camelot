use crate::domain::Percentage;

#[derive(Debug, Clone, PartialEq)]
pub enum SkillTrigger {
    PROC(Percentage),
    BasedOnCard(SkillTriggerValueFormula),
    ShiaiCondition(ShiaiCondition)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillTriggerValueFormula{
    TriggerBasedOnCardLevel(Percentage),
    TriggerBasedOnCardTier(Percentage)
}
#[derive(Debug, Clone, PartialEq)]
pub enum ShiaiCondition{
    EnemyHasSpy,
    OwnPartyMainlyDruid,
    OwnPartyMainlyCamelot
}