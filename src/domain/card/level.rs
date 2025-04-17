#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CardLevel(u8);

impl CardLevel {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn level_up(self, num_levels: u8) -> Self {
        Self(self.0 + num_levels)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let level = CardLevel::new(1);
        assert_eq!(level.value(), 1);
    }

    #[test]
    fn test_level_up() {
        let level = CardLevel::new(1);
        let level = level.level_up(3);
        assert_eq!(level.value(), 4);
    }
}