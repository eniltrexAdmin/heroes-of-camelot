#[derive(Debug, Clone, PartialEq)]
pub enum ComboSkillEffect {
    Passive(PassiveSkill),
    Active(SkillEffect),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PassiveSkill {
    AttackIncrease(u32),
    HealthPointsIncrease(u32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillEffect {
    IncreaseThisTurnAttack(ValueFormula),
    MagicDamage(ValueFormula)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueFormula{
    BasedOnCardAttack(u32),
    BasedOnCardHealthPoints(u32),
    BasedOnCardLevel(u32),
}

