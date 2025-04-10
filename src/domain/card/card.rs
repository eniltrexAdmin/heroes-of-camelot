use super::*;
use crate::domain::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    id: Id,
    name: Name,
    attack: Attack,
    health_points: HealthPoints,
    skills: Vec<Skill>,
}
impl Card {
    pub fn new(
        id: Id,
        name: Name,
        attack: Attack,
        health_points: HealthPoints,
        skills: Vec<Skill>,
    ) -> Self {
        Card {
            id,
            name,
            attack,
            health_points,
            skills,
        }
    }
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn attack(&self) -> &Attack {
        &self.attack
    }
    pub fn health_points(&self) -> &HealthPoints {
        &self.health_points
    }
    pub fn skills(&self) -> &Vec<Skill> {
        &self.skills
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let id = Id::new();
        let name = Name::new("Bard".to_string());
        let attack = Attack::new(100);
        let hp = HealthPoints::new(100);

        let passive_skill = Skill::new_passive(
            SkillName::new("Heavy body".to_string()),
            SkillDescription::new("Increases HP 100%".to_string()),
        );

        let skills = vec![passive_skill];

        let card = Card::new(id.clone(), name.clone(), attack, hp, skills.clone());

        assert_eq!(card.id, id);
        assert_eq!(card.name(), &name);
        assert_eq!(card.attack, attack);
        assert_eq!(card.health_points, hp);
        assert_eq!(card.skills, skills);
    }
}
