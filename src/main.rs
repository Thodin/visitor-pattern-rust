use std::fs;

use items::Bow;
use player::Player;

pub mod items;
pub mod player;

fn main() -> std::io::Result<()> {
    let player = Player {
        position: (50.0, 50.0),
        health: 100,
    };

    let bow = Bow {
        damage: 12,
        range: 18.3,
    };

    let output_dir = "../save_files";
    fs::create_dir_all(output_dir)?;

    player.save_to_file(&format!("{}/player.txt", output_dir))?;
    bow.save_to_file(&format!("{}/bow.txt", output_dir))?;

    Ok(())
}
