use crate::domain::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SkillEffect {
    IncreaseThisTurnAttack(SkillEffectValueFormula),
    DecreaseThisTurnAttack(SkillEffectValueFormula),
    MagicDamage(SkillEffectValueFormula),
    PhysicalDamage(SkillEffectValueFormula),
    Heal(SkillEffectValueFormula),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillEffectValueFormula {
    BasedOnCardAttack(Percentage),
    BasedOnCardHealthPoints(Percentage),
    EffectBasedOnCardLevel(Percentage),
}


