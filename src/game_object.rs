use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

use crate::{items::Bow, player::Player};

pub trait GameObjectVisitor {
    // TODO: you should do some error mapping on the return types -> Watch my custom errors video!
    fn visit_player(&self, player: &Player) -> std::io::Result<()>;
    fn visit_bow(&self, bow: &Bow) -> std::io::Result<()>;
}

pub trait GameObject {
    // accept visitor
    fn accept(&self, visitor: &dyn GameObjectVisitor) -> std::io::Result<()>;
}

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

pub struct ConsolePrinter {}

impl GameObjectVisitor for ConsolePrinter {
    fn visit_player(&self, _player: &Player) -> std::io::Result<()> {
        println!("This is the player...");
        Ok(())
    }

    fn visit_bow(&self, _bow: &Bow) -> std::io::Result<()> {
        println!("That's a bow!");
        Ok(())
    }
}
