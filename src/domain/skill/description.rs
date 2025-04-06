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
