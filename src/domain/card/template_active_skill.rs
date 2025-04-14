use std::collections::BTreeMap;
use crate::domain::*;

pub struct TemplateActiveSkills {
    value: BTreeMap<Tier, ActiveSkill>
}

impl TemplateActiveSkills {
    pub fn new_from_one(active_skill: ActiveSkill, vec_tier: Vec<Tier>) -> Self {
        let mut map = BTreeMap::new();
        for tier in vec_tier {
            map.insert(tier, active_skill.clone());
        }
        Self { value: map }
    }

    pub fn value(&self) -> &BTreeMap<Tier, ActiveSkill> {
        &self.value
    }
}




#[cfg(test)]
mod tests {
    use crate::domain::Stars::ThreeStars;
    use super::*;

    #[test]
    fn test_constructor_single() {
        let active_skill = ActiveSkill::new(
            SkillName::new("Some skill".to_string()),
            SkillDescription::new("Some description".to_string()),
            IncreaseThisTurnAttack(300)
        );

        let template_active_skill =
            TemplateActiveSkills::new_from_one(active_skill.clone(), Tier::vec_tier(&ThreeStars));

        assert_eq!(template_active_skill.value().len(), 4);
        assert!(template_active_skill.value().values().all(|s| s == &active_skill));
    }
}