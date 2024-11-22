use crate::game_object::{GameObject, GameObjectVisitor};

pub struct Player {
    pub position: (f32, f32),
    pub health: i32,
}

impl GameObject for Player {
    fn accept(&self, visitor: &dyn GameObjectVisitor) -> std::io::Result<()> {
        visitor.visit_player(&self)?;
        Ok(())
    }
}
