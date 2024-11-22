use crate::persistence::{Persistable, Persister};

pub struct Bow {
    pub damage: i32,
    pub range: f32,
}

impl Persistable for Bow {
    fn save_with(&self, saver: &dyn Persister) -> std::io::Result<()> {
        saver.save_bow(&self)?;
        Ok(())
    }
}
