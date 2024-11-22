use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use crate::{game_object::GameObjectVisitor, items::Bow, player::Player};

pub struct TxtFileSaver {
    save_dir: String,
}

impl TxtFileSaver {
    pub fn new(save_dir: String) -> std::io::Result<TxtFileSaver> {
        fs::create_dir_all(&save_dir)?;
        let txt_file_saver = TxtFileSaver { save_dir };
        Ok(txt_file_saver)
    }

    fn open_file(&self, struct_name: &str) -> std::io::Result<File> {
        let filename = format!("{}/{}.txt", self.save_dir, struct_name);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;
        Ok(file)
    }

    fn write_to_file(&self, struct_name: &str, content: String) -> std::io::Result<()> {
        let mut file = self.open_file(struct_name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl GameObjectVisitor for TxtFileSaver {
    fn visit_player(&self, player: &Player) -> std::io::Result<()> {
        let mut content = format!("Position: [{}, {}]\n", player.position.0, player.position.1);
        content.push_str(&format!("Health: {}", player.health));

        self.write_to_file("player", content)?;
        Ok(())
    }

    fn visit_bow(&self, bow: &Bow) -> std::io::Result<()> {
        let mut content = format!("Damage: {}\n", bow.damage);
        content.push_str(&format!("Range: {}", bow.range));

        self.write_to_file("bow", content)?;
        Ok(())
    }
}
