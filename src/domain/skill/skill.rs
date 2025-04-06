use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Skill {
    Passive(PassiveSkill),
    Active(ActiveSkill),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PassiveSkill {
    name: SkillName,
    description: SkillDescription,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveSkill {
    name: SkillName,
    description: SkillDescription,
}

impl Skill {
    pub fn new_passive(name: SkillName, description: SkillDescription) -> Self {
        Skill::Passive(PassiveSkill { name, description })
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

        let passive_skill = Skill::new_passive(name.clone(), description.clone());
        match passive_skill {
            Skill::Passive(ref passive) => {
                // Assert that the name and description are correctly set
                assert_eq!(name, passive.name);
                assert_eq!(description, passive.description);
            }
            _ => panic!("Expected a Passive skill!"),
        }
    }
}
