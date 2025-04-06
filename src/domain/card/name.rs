#[derive(Debug, PartialEq, Clone)]
pub struct Name(String);

impl Name {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    pub fn value(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    fn name_creation() {
        let value = "Elf spy".to_string();
        let name = Name::new(value.clone());
        assert_eq!(&value, name.value());
    }
}
