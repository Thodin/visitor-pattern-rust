use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::{items::Bow, player::Player};

// visitor
pub struct TxtFileSaver {
    save_dir: String,
}

impl TxtFileSaver {
    pub fn new(save_dir: String) -> std::io::Result<TxtFileSaver> {
        fs::create_dir_all(&save_dir)?;
        let txt_file_saver = TxtFileSaver { save_dir };
        Ok(txt_file_saver)
    }

    pub fn save_player(&self, player: &Player) -> std::io::Result<()> {
        let filename = format!("{}/player.txt", self.save_dir);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        file.write_all(
            format!("Position: [{}, {}]\n", player.position.0, player.position.1).as_bytes(),
        )?;
        file.write_all(format!("Health: {}", player.health).as_bytes())?;
        Ok(())
    }

    pub fn save_bow(&self, bow: &Bow) -> std::io::Result<()> {
        let filename = format!("{}/bow.txt", self.save_dir);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        file.write_all(format!("Damage: {}\n", bow.damage).as_bytes())?;
        file.write_all(format!("Range: {}", bow.range).as_bytes())?;
        Ok(())
    }
}
