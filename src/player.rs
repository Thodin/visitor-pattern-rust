use crate::persistence::{Persistable, Persister};

pub struct Player {
    pub position: (f32, f32),
    pub health: i32,
}

impl Persistable for Player {
    fn save_with(&self, saver: &dyn Persister) -> std::io::Result<()> {
        saver.save_player(&self)?;
        Ok(())
    }
}
