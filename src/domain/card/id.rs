use uuid::Uuid;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Id;
    use uuid::Uuid;

    #[test]
    fn id_creation() {
        let uuid = Uuid::new_v4();
        let id = Id::new();
        assert_ne!(&uuid, id.value());
    }
}
