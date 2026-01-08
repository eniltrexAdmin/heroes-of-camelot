use super::*;

pub type Percentage = u32;


#[derive(Debug, Clone, PartialEq)]
pub struct CardSkill {
    name: SkillName,
    description: SkillDescription,
    effect: SkillEffect,
    trigger: SkillTrigger,
    skill_target: SkillTarget
    //skill_duration: num of turns (belongs to skilleffect?)
    // skill_cooldown: num of turns (cannot trigger again.) - belongs to trigger?
    // skill_num_of_Triggers: once per battle/twice, etc.
}



impl CardSkill {
    pub fn new(
        name: SkillName,
        description: SkillDescription,
        effect: SkillEffect,
        trigger: SkillTrigger,
        skill_target: SkillTarget
    ) -> Self {
        CardSkill {
            name,
            description,
            effect,
            trigger,
            skill_target
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

    pub fn trigger(&self) -> &SkillTrigger {&self.trigger}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;

    #[test]
    fn skill_creation() {
        let value = "Heavy body".to_string();
        let name = SkillName::new(value.clone());
        let value = "Increases HP 100%".to_string();
        let description = SkillDescription::new(value.clone());
        let effect = SkillEffect::IncreaseThisTurnAttack(BasedOnCardAttack(100));
        let trigger = SkillTrigger::PROC(50);
        let skill_target = SkillTarget::Team(Itself);

        let skill = CardSkill::new(
            name.clone(),
            description.clone(),
            effect.clone(),
            trigger.clone(),
            skill_target.clone()
        );
        assert_eq!(&name, skill.name());
        assert_eq!(&description, skill.description());
        assert_eq!(&effect, skill.effect());
        assert_eq!(&trigger, skill.trigger());
        assert_eq!(skill_target, skill_target);
    }
}
