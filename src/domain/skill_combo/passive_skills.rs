use crate::domain::SkillEffect;

#[derive(Debug, Clone, PartialEq)]
pub enum ComboSkillEffect {
    Passive(PassiveSkill),
    Active(SkillEffect),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PassiveSkill {
    AttackIncrease(u32),
    HealthPointsIncrease(u32),
}
