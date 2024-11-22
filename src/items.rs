use crate::game_object::{GameObject, GameObjectVisitor};

pub struct Bow {
    pub damage: i32,
    pub range: f32,
}

impl GameObject for Bow {
    fn accept(&self, visitor: &dyn GameObjectVisitor) -> std::io::Result<()> {
        visitor.visit_bow(&self)?;
        Ok(())
    }
}
