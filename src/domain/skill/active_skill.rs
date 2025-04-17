use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CardSkill {
    name: SkillName,
    description: SkillDescription,
    effect: SkillEffect,
}

impl CardSkill {
    pub fn new(name: SkillName, description: SkillDescription, effect: SkillEffect) -> Self {
        CardSkill {
            name,
            description,
            effect
        }
    }

    pub fn name(&self) -> &SkillName {
        &self.name
    }

    pub fn description(&self) -> &SkillDescription {
        &self.description
    }

    pub fn effect(&self) -> &SkillEffect {
        &self.effect
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_creation() {
        let value = "Heavy body".to_string();
        let name = SkillName::new(value.clone());
        let value = "Increases HP 100%".to_string();
        let description = SkillDescription::new(value.clone());
        let effect = SkillEffect::IncreaseThisTurnAttack(BasedOnCardAttack(100));

        let skill = CardSkill::new(name.clone(), description.clone(), effect.clone());
        assert_eq!(&name, skill.name());
        assert_eq!(&description, skill.description());
        assert_eq!(&effect, skill.effect());
    }
}
