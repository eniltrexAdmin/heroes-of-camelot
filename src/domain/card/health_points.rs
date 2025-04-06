#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HealthPoints(u32);

impl HealthPoints {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
    pub fn value(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::HealthPoints;

    #[test]
    fn attack_creation() {
        let value = 3000;
        let resources_number = HealthPoints::new(value);
        assert_eq!(value, resources_number.value());
    }
}
