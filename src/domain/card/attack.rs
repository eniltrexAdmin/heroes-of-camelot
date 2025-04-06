#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Attack(u32);

impl Attack {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
    pub fn value(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Attack;

    #[test]
    fn attack_creation() {
        let value = 3000;
        let resources_number = Attack::new(value);
        assert_eq!(value, resources_number.value());
    }
}
