#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SkillBaseValue(u128);


impl SkillBaseValue {
    pub fn new(value: u128) -> Self {
        Self(value)
    }
    pub fn value(&self) -> u128 {
        self.0
    }
}