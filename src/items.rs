use std::{fs::OpenOptions, io::Write};

pub struct Bow {
    pub damage: i32,
    pub range: f32,
}

impl Bow {
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        file.write_all(format!("Damage: {}\n", self.damage).as_bytes())?;
        file.write_all(format!("Range: {}", self.range).as_bytes())?;
        Ok(())
    }
}
