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

    player.save_to_file("player.txt")?;
    bow.save_to_file("bow.txt")?;

    Ok(())
}
