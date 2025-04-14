#[derive(Debug, Clone, PartialEq)]
pub enum SkillEffect {
    Passive(PassiveSkill),
    Active(ActiveSkillEffect),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PassiveSkill {
    AttackIncrease(u32),
    HealthPointsIncrease(u32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActiveSkillEffect {
    IncreaseThisTurnAttack(u32)
}


