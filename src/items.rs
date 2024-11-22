use crate::persistence::Persister;

pub struct Bow {
    pub damage: i32,
    pub range: f32,
}

impl Bow {
    // accept visitor
    pub fn save_with(&self, saver: &dyn Persister) -> std::io::Result<()> {
        saver.save_bow(&self)?;
        Ok(())
    }
}
