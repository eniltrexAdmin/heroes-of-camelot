use crate::domain::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ComboSkill {
    name: SkillName,
    description: SkillDescription,
    effect: ComboSkillEffect,
    required_templates: Vec<Name>,
}

impl ComboSkill {
    pub fn new(
        name: SkillName,
        description: SkillDescription,
        effect: ComboSkillEffect,
        cards_team: Vec<Name>,
    ) -> Self {
        ComboSkill {
            name,
            description,
            effect,
            required_templates: cards_team,
        }
    }

    pub fn name(&self) -> &SkillName {
        &self.name
    }

    pub fn description(&self) -> &SkillDescription {
        &self.description
    }

    pub fn effect(&self) -> &ComboSkillEffect {
        &self.effect
    }

    pub fn required_templates(&self) -> &[Name] {
        &self.required_templates
    }
}

pub type ComboSkillFinder = fn(card: &Vec<Card>) -> Vec<ComboSkill>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_creation() {
        let value = "Heavy body".to_string();
        let name = SkillName::new(value.clone());
        let value = "Increases HP 100%".to_string();
        let description = SkillDescription::new(value.clone());
        let effect = ComboSkillEffect::Passive(AttackIncrease(100));

        let for_templates = vec![Name::new("card1".to_string())];

        let skill = ComboSkill::new(
            name.clone(),
            description.clone(),
            effect.clone(),
            for_templates.clone(),
        );
        assert_eq!(&name, skill.name());
        assert_eq!(&description, skill.description());
        assert_eq!(&effect, skill.effect());
        assert_eq!(&for_templates, skill.required_templates())
    }
}
