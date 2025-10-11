
#[derive(Debug, Clone, PartialEq)]
pub enum SkillTarget {
    Team(TeamSkillTarget),
    Party(PartySkillTarget)
}


#[derive(Debug, Clone, PartialEq)]
pub enum TeamSkillTarget {
    Itself,
    TeamTargetOwnParty(SkillTargetStrategy),
    TeamTargetEnemyParty(SkillTargetStrategy)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillTargetStrategy {
    Default,
    HighestCurrentHp,
    HighestAttack,
    LowestCurrentHp,
    LowestAttack,
}


#[derive(Debug, Clone, PartialEq)]
pub enum PartySkillTarget {
    OwnParty,
    EnemyParty
}