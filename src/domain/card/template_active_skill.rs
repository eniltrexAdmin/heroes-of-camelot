use crate::domain::*;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct TemplateSkills {
    value: BTreeMap<Tier, CardSkill>,
}

impl TemplateSkills {
    pub fn new_from_one(active_skill: CardSkill, vec_tier: Vec<Tier>) -> Self {
        let mut map = BTreeMap::new();
        for tier in vec_tier {
            map.insert(tier, active_skill.clone());
        }
        Self { value: map }
    }

    pub fn value(&self) -> &BTreeMap<Tier, CardSkill> {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Stars::ThreeStars;

    #[test]
    fn test_constructor_single() {
        let active_skill = CardSkill::new(
            SkillName::new("Some skill".to_string()),
            SkillDescription::new("Some description".to_string()),
            SkillEffect::IncreaseThisTurnAttack(BasedOnCardAttack(300)),
        );

        let template_active_skill =
            TemplateSkills::new_from_one(active_skill.clone(), Tier::vec_tier(&ThreeStars));

        assert_eq!(template_active_skill.value().len(), 4);
        assert!(template_active_skill
            .value()
            .values()
            .all(|s| s == &active_skill));
    }
}
