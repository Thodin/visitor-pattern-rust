use std::{fs::OpenOptions, io::Write};

pub struct Player {
    pub position: (f32, f32),
    pub health: i32,
}

impl Player {
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        file.write_all(
            format!("Position: [{}, {}]\n", self.position.0, self.position.1).as_bytes(),
        )?;
        file.write_all(format!("Health: {}", self.health).as_bytes())?;
        Ok(())
    }
}
