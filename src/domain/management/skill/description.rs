#[derive(Debug, PartialEq, Clone)]
pub struct SkillDescription(String);

impl SkillDescription {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    pub fn value(&self) -> &String {
        &self.0
    }
}

// TODO this should be calculated in the card. X change to X on team X. Per language
// the value belongs to the "card" not the template, but probably to just FE and not BE.
// so to the game engine, and maybe to display teh variables so the user knows
// which value belongs to, or well maybe too much, but sometinhg like "XX% chance
// to deal 200% attack damage to team leader".

#[cfg(test)]
mod tests {
    use super::SkillDescription;

    #[test]
    fn description_creation() {
        let value = "Increases HP 100%".to_string();
        let description = SkillDescription::new(value.clone());
        assert_eq!(&value, description.value());
    }
}
