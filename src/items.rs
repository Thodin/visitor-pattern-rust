use crate::game_object::{GameObject, GameObjectVisitor};

pub struct Bow {
    pub damage: i32,
    pub range: f32,
}

impl GameObject for Bow {
    fn accept(&self, saver: &dyn GameObjectVisitor) -> std::io::Result<()> {
        saver.visit_bow(&self)?;
        Ok(())
    }
}
