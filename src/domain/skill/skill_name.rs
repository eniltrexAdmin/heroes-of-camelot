#[derive(Debug, PartialEq, Clone)]
pub struct SkillName(String);

impl SkillName {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    pub fn value(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SkillName;

    #[test]
    fn name_creation() {
        let value = "Heavy body".to_string();
        let name = SkillName::new(value.clone());
        assert_eq!(&value, name.value());
    }
}
