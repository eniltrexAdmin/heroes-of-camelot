#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TeamHealthPoints(u128);

impl TeamHealthPoints {
    pub fn new(value: u128) -> Self {
        Self(value)
    }
    pub fn value(&self) -> u128 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::TeamHealthPoints;

    #[test]
    fn attack_creation() {
        let value = 3000;
        let resources_number = TeamHealthPoints::new(value);
        assert_eq!(value, resources_number.value());
    }
}
