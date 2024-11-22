use crate::persistence::TxtFileSaver;

pub struct Player {
    pub position: (f32, f32),
    pub health: i32,
}

impl Player {
    // accept visitor
    pub fn save_with(&self, saver: &TxtFileSaver) -> std::io::Result<()> {
        saver.save_player(&self)?;
        Ok(())
    }
}
